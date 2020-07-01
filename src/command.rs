use crate::{Context, Result, Error, ErrorKind, CommandResolver, CommandHandler,
    Flag, Param, Resource, Intent, build_subcommand_positions,
    build_command_summary, subcommand_at_position, build_supcommand_summaries,
    build_subcommand_summaries, build_flag_summaries, build_param_summaries,
    build_resource_summaries, parse_args};

/// Command structure which represents command-line task.
pub struct Command<C = Context> {
    name: String,
    about: Option<String>,
    description: Option<String>,
    author: Option<String>,
    version: Option<String>,
    flags: Vec<Flag>,
    params: Vec<Param>,
    resources: Vec<Resource>,
    commands: Vec<Command<C>>,
    handler: Option<CommandHandler<C>>,
    resolver: Option<CommandResolver<C>>,
}

/// Command structure implementation.
impl<C> Command<C> {

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

    /// Returns flags.
    pub fn params(&self) -> &Vec<Param> {
        &self.params
    }

    /// Returns resources.
    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    /// Returns commands.
    pub fn commands(&self) -> &Vec<Command<C>> {
        &self.commands
    }
}

/// Command structure implementation.
impl<C> Command<C> {

    /// Returns new instance.
    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            about: None,
            flags: Vec::new(),
            params: Vec::new(),
            resources: Vec::new(),
            commands: Vec::new(),
            handler: None,
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
    
    /// Sets error handler function.
    pub fn with_handler(mut self, handler: CommandHandler<C>) -> Self {
        self.handler = Some(handler);
        self
    }

    /// Sets resolver function.
    pub fn with_resolver(mut self, resolver: CommandResolver<C>) -> Self {
        self.resolver = Some(resolver);
        self
    }

    /// Adds flag.
    pub fn with_flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Adds param.
    pub fn with_param(mut self, param: Param) -> Self {
        self.params.push(param);
        self
    }

    /// Adds resource.
    pub fn with_resource(mut self, resource: Resource) -> Self {
        self.resources.push(resource);
        self
    }

    /// Adds subcommand.
    pub fn with_subcommand(mut self, command: Command<C>) -> Self {
        self.commands.push(command);
        self
    }

    /// Executes as a command-line application.
    pub fn run(self, ctx: &mut C) -> Result<usize> {
        self.run_args(parse_args(), ctx)
    }

    /// Executes as a command-line application.
    pub fn run_args<A, T>(self, args: A, ctx: &mut C) -> Result<usize>
        where
        A: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let args: Vec<String> = args.into_iter().map(Into::into).collect();
        let command_positions = build_subcommand_positions(&self, &args)?;
        let command = subcommand_at_position(&self, &command_positions);
        let command_summary = build_command_summary(&command);
        let supcommand_summaries = build_supcommand_summaries(&self, &command_positions);
        let subcommand_summaries = build_subcommand_summaries(&command);
        let flag_summaries = build_flag_summaries(&command, &args)?;
        let param_summaries = build_param_summaries(&self, &args)?;
        let resource_summaries = build_resource_summaries(&command);

        let intent = Intent::new(
            args,
            command_summary,
            supcommand_summaries,
            subcommand_summaries,
            flag_summaries,
            param_summaries,
            resource_summaries,
        );

        let err = match &command.resolver {
            Some(resolver) => match resolver(&intent, ctx) {
                Ok(code) => return Ok(code),
                Err(err) => err,
            },
            None => Error::new(ErrorKind::MissingCommandResolver(command.name().to_string())),
        };
        match &command.handler {
            Some(handler) => handler(err, &intent, ctx),
            None => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_command() {
        fn resolver(_: &Intent, _: &mut Context) -> Result<usize> { Ok(1) }
        let mut ctx = Context::default();
        let app = Command::with_name("a").with_resolver(resolver);
        assert_eq!(app.run_args(vec![] as Vec<String>, &mut ctx), Ok(1));
        let app = Command::with_name("a").with_resolver(|_, _| { Ok(2) });
        assert_eq!(app.run_args(vec![] as Vec<String>, &mut ctx), Ok(2));
    }

    #[test]
    fn resolves_subcommand() {
        fn resolver0(_: &Intent, _: &mut Context) -> Result<usize> { Ok(1) };
        fn resolver1(_: &Intent, _: &mut Context) -> Result<usize> { Ok(2) };
        let mut ctx = Context::default();
        let app = Command::with_name("a")
            .with_subcommand(Command::with_name("b").with_resolver(resolver0))
            .with_resolver(resolver1);
        assert_eq!(app.run_args(vec!["b"], &mut ctx), Ok(1));
    }

    #[test]
    fn handles_error() {
        fn resolver(_: &Intent, _: &mut Context) -> Result<usize> { Err(Error::default()) }
        fn handler(_error: Error, _: &Intent, _: &mut Context) -> Result<usize> { Ok(1) }
        let mut ctx = Context::default();
        let app = Command::with_name("a").with_resolver(resolver).with_handler(handler);
        assert_eq!(app.run_args(vec![] as Vec<String>, &mut ctx), Ok(1));
    }
}
