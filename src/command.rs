use crate::{ErrorCode, Resolver, Flag, Resource, Intent, build_subcommand_positions,
    build_command_summary, subcommand_at_position, build_supcommand_summaries,
    build_subcommand_summaries, build_flag_summaries, build_resource_summaries,
    parse_args};

/// Command structure which represents command-line task.
pub struct Command<'a> {
    name: String,
    about: Option<String>,
    description: Option<String>,
    author: Option<String>,
    version: Option<String>,
    flags: Vec<Flag>,
    resources: Vec<Resource>,
    commands: Vec<Command<'a>>,
    resolver: Option<&'a dyn Resolver>,
}

/// Command structure implementation.
impl <'a> Command<'a> {

    /// Returns name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns about.
    pub fn about(&self) -> &Option<String> {
        &self.about
    }

    /// Returns description.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Returns author.
    pub fn author(&self) -> &Option<String> {
        &self.author
    }

    /// Returns version.
    pub fn version(&self) -> &Option<String> {
        &self.version
    }

    /// Returns flags.
    pub fn flags(&self) -> &Vec<Flag> {
        &self.flags
    }

    /// Returns resources.
    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    /// Returns commands.
    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }
}

/// Command structure implementation.
impl <'a> Command<'a> {

    /// Returns new instance.
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            about: None,
            flags: Vec::new(),
            resources: Vec::new(),
            commands: Vec::new(),
            resolver: None,
            description: None,
            author: None,
            version: None,
        }
    }

    /// Sets about.
    pub fn with_about<S: Into<String>>(mut self, val: S) -> Self {
        self.about = Some(val.into());
        self
    }

    /// Sets description.
    pub fn with_description<S: Into<String>>(mut self, val: S) -> Self {
        self.description = Some(val.into());
        self
    }

    /// Sets about.
    pub fn with_author<S: Into<String>>(mut self, val: S) -> Self {
        self.author = Some(val.into());
        self
    }

    /// Sets version.
    pub fn with_version<S: Into<String>>(mut self, val: S) -> Self {
        self.version = Some(val.into());
        self
    }
    
    /// Sets resolver function.
    pub fn with_resolver(mut self, resolver: &'a dyn Resolver) -> Self {
        self.resolver = Some(resolver);
        self
    }

    /// Adds flag.
    pub fn with_flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Adds resource.
    pub fn with_resource(mut self, resource: Resource) -> Self {
        self.resources.push(resource);
        self
    }

    /// Adds subcommand.
    pub fn with_subcommand(mut self, command: Command<'a>) -> Self {
        self.commands.push(command);
        self
    }

    /// Executes as a command-line application.
    pub fn run(self) -> Result<usize, usize> {
        self.run_args(parse_args())
    }

    /// Executes as a command-line application.
    pub fn run_args<S: Into<String>>(self, args: Vec<S>) -> Result<usize, usize> {
        let args = args.into_iter().map(|s| s.into()).collect();
        let command_positions = match build_subcommand_positions(&self, &args) {
            Ok(v) => v,
            Err(code) => return Err(code),
        };
        let command = subcommand_at_position(&self, &command_positions);
        let command_summary = build_command_summary(&command);
        let supcommand_summaries = build_supcommand_summaries(&self, &command_positions);
        let subcommand_summaries = build_subcommand_summaries(&command);
        let flag_summaries = match build_flag_summaries(&command, &args) {
            Ok(v) => v,
            Err(code) => return Err(code),
        };
        let resource_summaries = build_resource_summaries(&command);

        let intent = Intent::new(
            args,
            command_summary,
            supcommand_summaries,
            subcommand_summaries,
            flag_summaries,
            resource_summaries,
        );

        let resolver = match &command.resolver {
            Some(resolver) => resolver,
            None => return Err(ErrorCode::MissingResolver as usize),
        };
        resolver.resolve(intent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performs_command() {
        struct Foo;
        impl Resolver for Foo {
            fn resolve(&self, _: Intent) -> Result<usize, usize> { Ok(1) }
        }
        fn resolver(_: Intent) -> Result<usize, usize> { Ok(2) };
        let app = Command::with_name("a").with_resolver(&Foo{});
        assert_eq!(app.run_args(vec![""]), Ok(1));
        let app = Command::with_name("a").with_resolver(&resolver);
        assert_eq!(app.run_args(vec![""]), Ok(2));
        let app = Command::with_name("a").with_resolver(&|_| { Ok(3) });
        assert_eq!(app.run_args(vec![""]), Ok(3));
    }

    #[test]
    fn performs_subcommand() {
        fn resolver0(_: Intent) -> Result<usize, usize> { Ok(1) };
        fn resolver1(_: Intent) -> Result<usize, usize> { Ok(2) };
        let app = Command::with_name("a")
            .with_subcommand(Command::with_name("b").with_resolver(&resolver0))
            .with_resolver(&resolver1);
        assert_eq!(app.run_args(vec!["b".to_string()]), Ok(1));
    }
}
