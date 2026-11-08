// atc-globus-shell — Terminal, CLI, Command Processor
pub mod shell;
pub mod cli_commands;
pub mod shell_history;
pub mod shell_completion;
pub mod pipe_system;

pub use shell::Shell;
pub use cli_commands::CommandRegistry;
pub use shell_history::ShellHistory;
pub use shell_completion::AutoComplete;
pub use pipe_system::PipeSystem;
