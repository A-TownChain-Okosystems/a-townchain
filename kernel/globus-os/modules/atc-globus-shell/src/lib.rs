// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// atc-globus-shell — Terminal, CLI, Command Processor
pub mod cli_commands;
pub mod pipe_system;
pub mod shell;
pub mod shell_completion;
pub mod shell_history;

pub use cli_commands::CommandRegistry;
pub use pipe_system::PipeSystem;
pub use shell::Shell;
pub use shell_completion::AutoComplete;
pub use shell_history::ShellHistory;
