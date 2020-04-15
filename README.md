> Command-line application framework.

Example:

```rs
use rawcmd::{Command, Flag, Intent};

fn resolver(intent: Intent) -> Option<usize>  {
    intent.command();
    intent.supcommands();
    intent.subcommands();
}

let app = Command::with_name("cmd1")
    .with_description("Command 1")
    .with_flag(
        Flag::with_name("flag1")
            .with_alias("f1")
            .with_description("Flag 1")
            .with_value(true, Some("default"))
    )
    .with_subcommand(
        Command::with_name("cmd1:1")
            .with_description("Command 1:1")
            .with_flag(
                Flag::with_name(flag2)
                    .with_alias("f2")
                    .with_description("Flag 2")
            )
            .with_resolver(resolver)
    )
    .with_resolver(|_| { Some(0) })
    .run();

match app {
    Ok(v) => println!("OK: {:?}", v),
    Err(v) => println!("Err: {:?}", v),
}
```

Notes:

* Enable "param=val" (with =) -> add `resolve_attributes()`
* Add term
