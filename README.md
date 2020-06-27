> Command-line application framework.

## Example

A simple command-line application could look something like this:

```rs
use rawcmd::{Command, Flag, Intent};

fn main() {
    match Command::with_name("foo")
        .with_description("Command 1")
        .with_flag(
            Flag::with_name("flag1")
                .with_alias("f1")
                .with_description("Flag 1")
        )
        .with_subcommand(
            Command::with_name("bar")
                .with_description("Command 1:1")
                .with_flag(
                    Flag::with_name("flag2")
                        .with_alias("f2")
                        .with_description("Flag 2")
                )
                .with_resolver(|_| Ok(2))
        )
        .with_resolver(|_| Ok(3))
        .run()
    {
        Ok(code) => {
            println!("Success: {:?}", code);
            std::process::exit(0);
        },
        Err(error) => {
            println!("Error: {:?}", error);
            std::process::exit(1);
        },
    }
}
```

## TO-DO

* Support command inputs (e.g. `cli <COMMAND> <FLAG> <INPUT0> <INPUT1>`).
