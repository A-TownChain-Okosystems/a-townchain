//! Length-Prefix-Framing: [u32 BE Länge][Payload]. Fail-Closed-Parsing.

#[derive(Debug, PartialEq)]
pub enum TransportError {
    Incomplete { expected: usize, available: usize },
    LengthMismatch { declared: usize, actual: usize },
    Oversized { declared: usize, limit: usize },
}

pub const MAX_FRAME: usize = 4 * 1024 * 1024;

/// Frame kodieren.
pub fn frame(payload: &[u8]) -> Vec<u8> {
    let len = payload.len() as u32;
    let mut out = Vec::with_capacity(4 + payload.len());
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(payload);
    out
}

/// Ersten Frame aus einem Buffer parsen; gibt (Payload, konsumierte Bytes) zurück.
pub fn parse_frame(buf: &[u8]) -> Result<(&[u8], usize), TransportError> {
    if buf.len() < 4 {
        return Err(TransportError::Incomplete {
            expected: 4,
            available: buf.len(),
        });
    }
    let declared = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
    if declared > MAX_FRAME {
        return Err(TransportError::Oversized {
            declared,
            limit: MAX_FRAME,
        });
    }
    if buf.len() - 4 < declared {
        return Err(TransportError::Incomplete {
            expected: 4 + declared,
            available: buf.len(),
        });
    }
    Ok((&buf[4..4 + declared], 4 + declared))
}
