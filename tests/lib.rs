use rawcmd::{Command};

#[test]
fn performs_command() {
    let result = Command::with_name("0")
        .with_subcommand(
            Command::with_name("1").with_resolver(|_| Ok(0))
        )
        .with_subcommand(
            Command::with_name("2").with_resolver(|_| Ok(1))
        )
        .with_subcommand(
            Command::with_name("3").with_resolver(|_| Ok(0))
        )
        .with_resolver(|_| Ok(0))
        .perform(
            vec!["2".to_string()],
        );
    assert_eq!(result, Ok(1));
}
