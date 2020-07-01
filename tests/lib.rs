use rawcmd::{Context, Command, Intent, Result};

#[test]
fn performs_command() {
    fn resolver0(_: &Intent, _: &mut Context) -> Result<i32> { Ok(0) };
    fn resolver1(_: &Intent, _: &mut Context) -> Result<i32> { Ok(1) };
    let result = Command::with_name("0")
        .with_subcommand(
            Command::with_name("1").with_resolver(resolver0)
        )
        .with_subcommand(
            Command::with_name("2").with_resolver(resolver1)
        )
        .with_subcommand(
            Command::with_name("3").with_resolver(resolver0)
        )
        .with_resolver(resolver0)
        .run_args(vec!["2"], &mut Context::default());
    assert_eq!(result, Ok(1));
}
