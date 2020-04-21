use rawcmd::{Command, Resolver, Intent};

#[test]
fn performs_command() {
    struct Foo;
    impl Resolver for Foo {
        fn resolve(&self, _: Intent) -> Result<usize, usize> { Ok(1) }
    }
    fn bar(_: Intent) -> Result<usize, usize> { Ok(2) };
    let result = Command::with_name("0")
        .with_subcommand(
            Command::with_name("1").with_resolver(&Foo{})
        )
        .with_subcommand(
            Command::with_name("2").with_resolver(&Foo{})
        )
        .with_subcommand(
            Command::with_name("3").with_resolver(&Foo{})
        )
        .with_resolver(&bar)
        .run_args(
            vec!["2".to_string()],
        );
    assert_eq!(result, Ok(1));
}
