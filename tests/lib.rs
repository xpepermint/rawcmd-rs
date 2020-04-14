use rawcmd::{Command, Intent};

fn resolver0(_: Intent) -> Option<usize> {
    Some(0)
}
fn resolver1(_: Intent) -> Option<usize> {
    Some(1)
}

#[test]
fn performs_command() {
    let app = Command::with_name("0")
        .with_subcommand(
            Command::with_name("1").with_resolver(resolver1)
        )
        .with_resolver(resolver0);
    let args = vec![];
    assert_eq!(app.perform(args), Some(0));
}

#[test]
fn performs_subcommand() {
    let app = Command::with_name("0")
        .with_subcommand(
            Command::with_name("1").with_resolver(resolver0)
        )
        .with_subcommand(
            Command::with_name("2").with_resolver(resolver1)
        )
        .with_subcommand(
            Command::with_name("3").with_resolver(resolver0)
        )
        .with_resolver(resolver0);
    let args = vec!["2".to_string()];
    assert_eq!(app.perform(args), Some(1));
}
