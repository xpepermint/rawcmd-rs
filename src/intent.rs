use crate::{CommandSummary, FlagSummary};

/// Intent structure which represents user intent.
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    args: Vec<String>,
    command: CommandSummary,
    supcommands: Vec<CommandSummary>,
    subcommands: Vec<CommandSummary>,
    flags: Vec<FlagSummary>,
}

/// Intent structure implementation.
impl Intent {

    /// Returns new instance.
    pub fn new(
        args: Vec<String>,
        command: CommandSummary,
        supcommands: Vec<CommandSummary>,
        subcommands: Vec<CommandSummary>,
        flags: Vec<FlagSummary>,
    ) -> Self {
        Self {
            args,
            command,
            supcommands,
            subcommands,
            flags,
        }
    }

    /// Returns raw command-line arguments.
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    /// Returns summary objects of the executed command.
    pub fn command(&self) -> &CommandSummary {
        &self.command
    }

    /// Returns summary objects of parent commands in a tree.
    pub fn supcommands(&self) -> &Vec<CommandSummary> {
        &self.supcommands
    }

    /// Returns summary objects of child commands.
    pub fn subcommands(&self) -> &Vec<CommandSummary>{
        &self.subcommands
    }

    /// Returns summary objects of all flags.
    pub fn flags(&self) -> &Vec<FlagSummary> {
        &self.flags
    }

    /// Returns true if command-line arguments are present.
    pub fn has_args(&self) -> bool {
        !self.args.is_empty()
    }

    /// Returns true if the executed command has parent commands.
    pub fn has_supcommands(&self) -> bool {
        !self.supcommands.is_empty()
    }

    /// Returns true if the executed command has child commands.
    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
    }

    /// Returns true if the executed command has child commands.
    pub fn has_flags(&self) -> bool {
        !self.flags.is_empty()
    }
}
