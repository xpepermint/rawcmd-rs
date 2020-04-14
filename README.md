> Command-line application framework.

Example:

```rs
use std::env;
use rawcmd::{Command, Flag, Intent};

fn resolver1(intent: Intent) {}
fn resolver2(intent: Intent) {}

Command::new("cmd1")
    .with_description("Command 1")
    .with_flag(
        Flag::new("flag1")
            .with_alias("f1")
            .with_description("Flag 1")
            .with_value(true, Some("default"))
    )
    .with_subcommand(
        Command::new("cmd1:1")
            .with_description("Command 1:1")
            .with_flag(
                Flag::new(flag2)
                    .with_alias("f2")
                    .with_description("Flag 2")
            )
            .with_resolver(resolver2)
    )
    .with_resolver(resolver1)
    .perform(
        env::args().skip(1).collect(),
    );
```
