// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — Serielle Debug-Konsole (QEMU: -serial stdio)
// Wird für automatisierte Boot-Tests genutzt, bevor Tastatur/GUI existieren.

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{backend::PioBackend, Config, Uart16550Tty};

lazy_static! {
    pub static ref SERIAL1: Mutex<Uart16550Tty<PioBackend>> = {
        // SAFETY: COM1 (0x3F8) is the standard x86 PC serial port and is exclusively
        // owned by the kernel while this device instance is alive.
        let serial_port = unsafe {
            Uart16550Tty::new_port(0x3F8, Config::default())
        }
        .expect("COM1 UART initialization failed");
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Serieller Ausgabe-Fehler");
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
