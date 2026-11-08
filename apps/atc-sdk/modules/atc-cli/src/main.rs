// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
use std::env;

fn print_help() {
    println!("atc-cli — A-TownChain-Okosystems");
    println!();
    println!("Usage: atc-cli [--help] [--version]");
    println!();
    println!("Commands that require RPC/chain integration are exposed by the library modules and will be added only with a verified backend contract.");
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("--version") | Some("-V") => println!("atc-cli 0.1.0"),
        Some(command) => {
            eprintln!("unknown command: {command}");
            eprintln!("use --help for supported invocation");
            std::process::exit(2);
        }
    }
}
