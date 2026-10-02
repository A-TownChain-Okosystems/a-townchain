// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! R1CS-lite: Constraint-Check gegen Witness-Belegung (MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub a: Vec<(usize, u64)>,
    pub b: Vec<(usize, u64)>,
    pub c: Vec<(usize, u64)>,
}

fn eval(lc: &[(usize, u64)], witness: &[u64]) -> Option<u64> {
    let mut acc = 0u64;
    for (i, w) in lc {
        let value = witness.get(*i)?;
        acc = acc.wrapping_add(w.wrapping_mul(*value));
    }
    Some(acc)
}

pub fn satisfied(con: &Constraint, witness: &[u64]) -> bool {
    let a = match eval(&con.a, witness) {
        Some(value) => value,
        None => return false,
    };
    let b = match eval(&con.b, witness) {
        Some(value) => value,
        None => return false,
    };
    let c = match eval(&con.c, witness) {
        Some(value) => value,
        None => return false,
    };
    a.wrapping_mul(b) == c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quadrat_constraint() {
        // x * x == x  (fuer x in {0,1})
        let con = Constraint { a: vec![(0, 1)], b: vec![(0, 1)], c: vec![(0, 1)] };
        assert!(satisfied(&con, &[1]));
        assert!(satisfied(&con, &[0]));
        assert!(!satisfied(&con, &[2]));
    }

    #[test]
    fn out_of_bounds_witness_is_unsatisfied_not_panic() {
        let con = Constraint {
            a: vec![(1, 1)],
            b: vec![(0, 1)],
            c: vec![(0, 1)],
        };
        assert!(!satisfied(&con, &[1]));
    }
}
