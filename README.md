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
        Ok(code) => println!("Success: {:?}", code),
        Err(code) => println!("Error: {:?}", code),
    }
}
```

The function `with_resolver` accepts closures and function pointers:

```rs
// closure
command.with_resolver(|_| Ok(1))

// function pointer
fn resolver(_: Intent) -> Result<usize, usize> { Ok(2) }
command.with_resolver(&resolver)
```

## TO-DO

* Support command inputs (e.g. `cli command <input0> <input1>`).
