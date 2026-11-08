//! Echter TCP-Transport (G2-C, Issue #112) — Socket-Schicht über dem
//! bestehenden Length-Prefix-Framing (transport.rs, fail-closed).
//!
//! Vertrag:
//! - **Frame-Disziplin:** jede Nachricht ist [u32 BE Länge][Payload];
//!   `MAX_FRAME` (4 MiB) begrenzt den Speicher — Oversized = Verbindung
//!   fail-closed beenden (Backpressure über die Grenze, nicht durch RAM).
//! - **Kein Blind-Read:** `receive()` liest genau EINEN Frame; unvollständige
//!   Frames bleiben im Puffer (Stream != Nachricht).
//! - **EOF mitten im Frame** = Protokollverstoß → `ConnectionClosed`-Fehler,
//!   keine stillen Teilzustände.

use crate::transport::{frame, parse_frame, TransportError};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
pub enum TcpError {
    Io(std::io::Error),
    Frame(TransportError),
    /// Verbindung mitten in einem Frame beendet — Protokollverstoß.
    ClosedMidFrame,
}

impl std::fmt::Display for TcpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TcpError::Io(e) => write!(f, "tcp io: {e}"),
            TcpError::Frame(e) => write!(f, "frame: {e:?}"),
            TcpError::ClosedMidFrame => write!(f, "verbindung mitten im Frame beendet"),
        }
    }
}

/// Ein TCP-Peer mit Frame-Puffer (eine Nachricht = ein Frame).
pub struct TcpPeer {
    stream: TcpStream,
    buf: Vec<u8>,
}

impl TcpPeer {
    pub fn connect(addr: &str, timeout: Duration) -> Result<Self, TcpError> {
        let stream = TcpStream::connect(addr).map_err(TcpError::Io)?;
        stream
            .set_read_timeout(Some(timeout))
            .map_err(TcpError::Io)?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(TcpError::Io)?;
        stream.set_nodelay(true).map_err(TcpError::Io)?;
        Ok(Self {
            stream,
            buf: Vec::new(),
        })
    }

    /// Von einem accept() übernommener Stream.
    pub fn from_stream(stream: TcpStream, timeout: Duration) -> Result<Self, TcpError> {
        stream
            .set_read_timeout(Some(timeout))
            .map_err(TcpError::Io)?;
        stream
            .set_write_timeout(Some(timeout))
            .map_err(TcpError::Io)?;
        stream.set_nodelay(true).map_err(TcpError::Io)?;
        Ok(Self {
            stream,
            buf: Vec::new(),
        })
    }

    pub fn peer_addr(&self) -> String {
        self.stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_default()
    }

    /// Payload als Frame senden — vollständiges Schreiben oder Fehler.
    pub fn send(&mut self, payload: &[u8]) -> Result<(), TcpError> {
        let framed = frame(payload);
        self.stream.write_all(&framed).map_err(TcpError::Io)?;
        self.stream.flush().map_err(TcpError::Io)?;
        Ok(())
    }

    /// Genau EINEN Frame empfangen (blockierend bis Frame komplett).
    /// Unvollständige Daten bleiben im Puffer für den nächsten Aufruf.
    pub fn receive(&mut self) -> Result<Vec<u8>, TcpError> {
        loop {
            // Puffer schon komplett? Dann Frame ausschneiden und Rest behalten.
            if let Ok((payload, consumed)) = parse_frame(&self.buf) {
                let out = payload.to_vec();
                self.buf.drain(..consumed);
                return Ok(out);
            }
            // Lesen (nur wenn der Puffer noch keinen vollständigen Frame hat)
            let mut chunk = [0u8; 8192];
            let n = self.stream.read(&mut chunk).map_err(TcpError::Io)?;
            if n == 0 {
                return Err(TcpError::ClosedMidFrame);
            }
            self.buf.extend_from_slice(&chunk[..n]);
            // Oversized-Fail-Closed explizit prüfen (parse_frame liefert Err nur bei Incomplete hier)
            if let Err(TransportError::Oversized { declared, limit }) = parse_frame(&self.buf) {
                return Err(TcpError::Frame(TransportError::Oversized {
                    declared,
                    limit,
                }));
            }
        }
    }
}
