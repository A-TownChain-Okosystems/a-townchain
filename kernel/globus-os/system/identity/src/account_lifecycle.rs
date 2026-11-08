//! Account-lifecycle primitives that are safe to compose with the identity service.
//!
//! This module deliberately does not own wallet cryptography, persistent account directories,
//! or OS authorization. Those remain separate integration boundaries.

use argon2::password_hash::rand_core::{OsRng, RngCore};
use bip39::{Language, Mnemonic};
use zeroize::Zeroizing;

/// Canonical local username: 3..=32 ASCII characters, case-insensitive, [a-z0-9_.-].
///
/// Unicode look-alikes and whitespace are rejected to prevent ambiguous account identifiers.
/// The normalized value is suitable as a directory key, but global uniqueness still requires
/// an atomic persistent account-directory implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountUsername(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountLifecycleError {
    InvalidUsername,
    InvalidRecoveryPhrase,
    InvalidConfirmation,
}

impl AccountUsername {
    pub fn parse(value: &str) -> Result<Self, AccountLifecycleError> {
        let normalized = value.to_ascii_lowercase();
        let bytes = normalized.as_bytes();
        if !(3..=32).contains(&bytes.len())
            || !bytes.iter().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(*byte, b'_' | b'.' | b'-')
            })
            || !bytes[0].is_ascii_alphanumeric()
            || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
        {
            return Err(AccountLifecycleError::InvalidUsername);
        }
        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A one-time challenge requiring confirmation of three randomly selected recovery words.
///
/// Store this object only for the short-lived registration ceremony. The expected words are
/// zeroized on drop. The caller must not activate the account until verify() succeeds.
pub struct RecoveryConfirmation {
    positions: [usize; 3],
    expected: [Zeroizing<String>; 3],
}

impl RecoveryConfirmation {
    pub fn new(phrase: &str) -> Result<Self, AccountLifecycleError> {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        if words.len() != 24
            || words.iter().any(|word| word.is_empty())
            || Mnemonic::parse_in_normalized(Language::English, phrase).is_err()
        {
            return Err(AccountLifecycleError::InvalidRecoveryPhrase);
        }

        // Rejection sampling avoids modulo bias and guarantees three distinct positions.
        let mut positions = [0usize; 3];
        let zone = u32::MAX - (u32::MAX % words.len() as u32);
        for index in 0..positions.len() {
            loop {
                let random = OsRng.next_u32();
                if random >= zone {
                    continue;
                }
                let candidate = (random % words.len() as u32) as usize;
                if !positions[..index].contains(&candidate) {
                    positions[index] = candidate;
                    break;
                }
            }
        }
        let expected =
            std::array::from_fn(|index| Zeroizing::new(words[positions[index]].to_owned()));
        Ok(Self {
            positions,
            expected,
        })
    }

    /// Zero-based word positions. UI should display these as one-based positions.
    pub fn positions(&self) -> [usize; 3] {
        self.positions
    }

    /// Answers must be supplied in the same order as positions(). Comparison is
    /// case-insensitive; surrounding whitespace is ignored.
    pub fn verify(&self, answers: [&str; 3]) -> Result<(), AccountLifecycleError> {
        let valid = answers.iter().enumerate().all(|(index, answer)| {
            answer
                .trim()
                .eq_ignore_ascii_case(self.expected[index].as_str())
        });
        if valid {
            Ok(())
        } else {
            Err(AccountLifecycleError::InvalidConfirmation)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_is_canonical_and_case_insensitive() {
        assert_eq!(
            AccountUsername::parse("Alice_01").unwrap().as_str(),
            "alice_01"
        );
        assert_eq!(
            AccountUsername::parse("ALICE_01").unwrap(),
            AccountUsername::parse("alice_01").unwrap()
        );
    }

    #[test]
    fn username_rejects_ambiguous_or_invalid_values() {
        let too_long = "a".repeat(33);
        for value in [
            "",
            "ab",
            " leading",
            "a b c",
            "-alice",
            "alice-",
            "älice",
            too_long.as_str(),
        ] {
            assert_eq!(
                AccountUsername::parse(value),
                Err(AccountLifecycleError::InvalidUsername)
            );
        }
    }

    #[test]
    fn recovery_confirmation_accepts_only_selected_words() {
        let phrase = format!("{} art", vec!["abandon"; 23].join(" "));
        let challenge = RecoveryConfirmation::new(&phrase).unwrap();
        let positions = challenge.positions();
        assert!(positions.iter().all(|position| *position < 24));
        assert_ne!(positions[0], positions[1]);
        assert_ne!(positions[0], positions[2]);
        assert_ne!(positions[1], positions[2]);
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let answers = [
            words[positions[0]],
            words[positions[1]],
            words[positions[2]],
        ];
        assert!(challenge.verify(answers).is_ok());
        assert_eq!(
            challenge.verify(["wrong", "wrong", "wrong"]),
            Err(AccountLifecycleError::InvalidConfirmation)
        );
    }

    #[test]
    fn recovery_confirmation_rejects_non_24_word_phrase() {
        assert!(matches!(
            RecoveryConfirmation::new("abandon abandon"),
            Err(AccountLifecycleError::InvalidRecoveryPhrase)
        ));
    }
}
