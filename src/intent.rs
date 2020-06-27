use crate::{CommandSummary, FlagSummary, ResourceSummary};

/// Intent structure which represents user intent.
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    args: Vec<String>,
    command: CommandSummary,
    supcommands: Vec<CommandSummary>,
    subcommands: Vec<CommandSummary>,
    flags: Vec<FlagSummary>,
    resources: Vec<ResourceSummary>,
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
        resources: Vec<ResourceSummary>,
    ) -> Self {
        Self {
            args,
            command,
            supcommands,
            subcommands,
            flags,
            resources
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

    /// Returns summary objects of a specific flag.
    pub fn flag<S: Into<String>>(&self, name: S) -> Option<&FlagSummary> {
        let name = name.into();
        self.flags.iter().find(|f| *f.name() == name)
    }

    /// Returns summary objects of all resources.
    pub fn resources(&self) -> &Vec<ResourceSummary> {
        &self.resources
    }

    /// Returns summary objects of a specific resource.
    pub fn resource<S: Into<String>>(&self, name: S) -> Option<&ResourceSummary> {
        let name = name.into();
        self.resources.iter().find(|f| *f.name() == name)
    }

    /// Returns true if command-line arguments are present.
    pub fn has_args(&self) -> bool {
        !self.args.is_empty()
    }

    /// Returns true if command-line argument is present.
    pub fn has_arg<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        self.args.iter().any(|a| *a == name)
    }

    /// Returns true if the executed command has parent commands.
    pub fn has_supcommands(&self) -> bool {
        !self.supcommands.is_empty()
    }

    /// Returns true if the executed command has child commands.
    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
    }

    /// Returns true if the executed command has flags.
    pub fn has_flags(&self) -> bool {
        !self.flags.is_empty()
    }

    /// Returns true if flag is present.
    pub fn has_flag<S: Into<String>>(&self, name: S) -> bool {
        self.flag(name.into()).is_some()
    }

    /// Returns true if flag is present.
    pub fn has_provided_flag<S: Into<String>>(&self, name: S) -> bool {
        let name = name.into();
        match self.flag(name) {
            Some(f) => f.provided(),
            None => false,
        }
    }

    /// Returns true if the executed command has resources.
    pub fn has_resources(&self) -> bool {
        !self.resources.is_empty()
    }

    /// Returns true if resource is present.
    pub fn has_resource<S: Into<String>>(&self, name: S) -> bool {
        self.resource(name.into()).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intent_with_args(args: Vec<String>) -> Intent {
        let command: CommandSummary = CommandSummary::with_name("", None, None, None, None);
        let supcommands: Vec<CommandSummary> = vec![];
        let subcommands: Vec<CommandSummary> = vec![];
        let flags: Vec<FlagSummary> = vec![];
        let resources: Vec<ResourceSummary> = vec![];
        Intent::new(args, command, supcommands, subcommands, flags, resources)
    }

    fn intent_with_flags(flags: Vec<FlagSummary>) -> Intent {
        let args: Vec<String> = vec![];
        let command: CommandSummary = CommandSummary::with_name("", None, None, None, None);
        let supcommands: Vec<CommandSummary> = vec![];
        let subcommands: Vec<CommandSummary> = vec![];
        let flags: Vec<FlagSummary> = flags;
        let resources: Vec<ResourceSummary> = vec![];
        Intent::new(args, command, supcommands, subcommands, flags, resources)
    }

    #[test]
    fn provides_flag_by_name() {
        let intent = intent_with_flags(vec![
            FlagSummary::with_name("a", None, None, None, None, false, false),
            FlagSummary::with_name("b", None, None, None, None, false, false),
        ]);
        assert_eq!(intent.flag("b").unwrap().name(), "b");
    }

    #[test]
    fn checks_argument_existance() {
        let intent = intent_with_args(vec![
            "--b".to_string(), "-c".to_string(),
        ]);
        assert_eq!(intent.has_arg("-c"), true);
        assert_eq!(intent.has_arg("b"), false);
    }

    #[test]
    fn checks_flag_existance() {
        let intent = intent_with_flags(vec![
            FlagSummary::with_name("b", None, None, None, None, false, false),
        ]);
        assert_eq!(intent.has_flag("b"), true);
        assert_eq!(intent.has_flag("x"), false);
    }

}
