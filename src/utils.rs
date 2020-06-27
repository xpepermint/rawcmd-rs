use std::env;
use crate::{Result, Error, ErrorKind, Command, CommandSummary, Flag, FlagSummary,
    Resource, ResourceSummary};

/// Parses command-line arguments.
pub fn parse_args() -> Vec<String> {
    let args = env::args().skip(1).collect();
    split_equal_args(&args)
}

/// Parses command-line arguments.
pub fn split_equal_args(args: &Vec<String>) -> Vec<String> {
    let items: Vec<Vec<String>> = args.iter()
        .map(|a| a.splitn(2, '=').map(|s| s.to_string()).collect())
        .collect();
    let items: Vec<String> = items.iter()
        .flat_map(|tup| tup.iter())
        .cloned()
        .collect();
    items
}

/// Parses arguments and finds command positions in a tree.
pub fn build_subcommand_positions(app: &Command, args: &Vec<String>) -> Result<Vec<usize>> {
    let mut args = args.clone();
    args.reverse();

    let mut positions: Vec<usize> = Vec::new();
    let mut command = app;
    loop {
        let arg = match args.pop() {
            None => break,
            arg => arg.unwrap(),
        };
        if arg.starts_with("-") {
            break;
        }

        let commands = &command.commands();
        let size = &commands.len();
        for (index, cmd) in commands.iter().enumerate() {
            command = cmd;
            if *cmd.name() == arg {
                positions.push(index);
                break;
            } else if index == size - 1 {
                return Err(Error::new(ErrorKind::UnknownCommand(arg)));
            }
        }
    }

    Ok(positions)
}

/// Returns command object based on the position in arguments.
pub fn subcommand_at_position<'a>(app: &'a Command, positions: &Vec<usize>) -> &'a Command {
    let mut command = app;
    for position in positions.clone().into_iter() {
        command = &command.commands().get(position).unwrap();
    }
    command
}

/// Returns command summary.
pub fn build_command_summary(command: &Command) -> CommandSummary {
    CommandSummary::with_name(
        command.name().clone().as_str(),
        command.about().clone(),
        command.description().clone(),
        command.author().clone(),
        command.version().clone(),
    )
}

/// Returns command summary.
pub fn build_flag_summary(flag: &Flag, provided: bool, value: &Option<String>) -> FlagSummary {
    FlagSummary::with_name(
        flag.name().clone().as_str(),
        flag.alias().clone(),
        flag.description().clone(),
        value.clone(),
        flag.default_value().clone(),
        flag.accepts_value().clone(),
        provided.clone(),
    )
}

/// Returns resource summary.
pub fn build_resource_summary(resource: &Resource) -> ResourceSummary {
    ResourceSummary::with_name(
        resource.name().clone().as_str(),
        resource.description().clone(),
    )
}

/// Returns summary objects of parent commands. 
pub fn build_supcommand_summaries(app: &Command, positions: &Vec<usize>) -> Vec<CommandSummary> {
    let mut items = Vec::new();
    items.push(build_command_summary(&app));

    let mut command = app;
    for position in positions.clone().into_iter() {
        command = &command.commands().get(position).unwrap();
        items.push(build_command_summary(&command));
    }
    items.pop();
    items
}

/// Returns summary objects of child commands. 
pub fn build_subcommand_summaries(command: &Command) -> Vec<CommandSummary> {
    let mut items = Vec::new();
    for subcommand in command.commands().into_iter() {
        items.push(build_command_summary(subcommand));
    }
    items.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));
    items
}

/// Returns flag summary objects for command. 
pub fn build_flag_summaries(command: &Command, args: &Vec<String>) -> Result<Vec<FlagSummary>> {
    let mut items = Vec::new();

    for (index, arg) in args.into_iter().enumerate() {

        if !arg.starts_with("-") {
            continue
        } else if arg == "--" {
            break;
        }

        let flag = match command.flags().iter().find(|&f| {
            *arg == format!("{}{}", "--", &f.name())
            || f.alias().is_some() && *arg == format!("{}{}", "-", f.alias().as_ref().unwrap())
        }) {
            Some(f) => f,
            None => return Err(Error::new(ErrorKind::UnknownFlag(arg.to_string()))),
        };

        let value = match flag.accepts_value() {
            true => match args.get(index + 1) {
                Some(value) => match value.starts_with("-") {
                    true => return Err(Error::new(ErrorKind::MissingFlagValue(arg.to_string()))),
                    false => Some(value.to_string()),
                },
                None => return Err(Error::new(ErrorKind::MissingFlagValue(arg.to_string()))),
            },
            false => None,
        };
        items.push(build_flag_summary(flag, true, &match flag.resolver() {
            Some(resolve) => resolve(value)?,
            None => value,
        }));
    }
 
    for flag in command.flags().into_iter() {
        let exists = &items.iter().any(|f| f.name() == flag.name());
        if !exists {
            let value = match flag.default_value() {
                Some(v) => Some(v.to_string()),
                None => None,
            };
            items.push(build_flag_summary(flag, false, &match flag.resolver() {
                Some(resolve) => resolve(value)?,
                None => value,
            }));
        }
    }
    items.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));
    Ok(items)
}

/// Returns resource summary objects for command. 
pub fn build_resource_summaries(command: &Command) -> Vec<ResourceSummary> {
    command.resources().iter().map(|r| {
        build_resource_summary(r)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_equal_args() {
        assert_eq!(
            split_equal_args(&vec!["aa", "bb=11", "--cc=22", "-d=33"].iter().map(|s| s.to_string()).collect()),
            vec!["aa", "bb", "11", "--cc", "22", "-d", "33"],
        );
    }

    #[test]
    fn builds_command_positions() {
        let command = Command::with_name("000")
            .with_subcommand(
                Command::with_name("aaa")
                    .with_subcommand(Command::with_name("bbb"))
                    .with_subcommand(Command::with_name("ccc"))
        );
        let args = vec!["aaa".to_string(), "ccc".to_string()];
        let positions = build_subcommand_positions(&command, &args).unwrap();
        let total = positions.len();
        assert_eq!(total, 2);
        assert_eq!(positions, [0, 1]);
    }

    #[test]
    fn builds_supcommand_summaries() {
        let command = Command::with_name("000")
            .with_subcommand(
                Command::with_name("aaa")
                    .with_subcommand(
                        Command::with_name("bbb")
                    )
            );
        let args = vec!["aaa".to_string(), "bbb".to_string()];
        let positions = build_subcommand_positions(&command, &args).unwrap();
        let summaries = build_supcommand_summaries(&command, &positions);
        let names: Vec<String> = summaries.iter()
            .map(|s| s.name().clone()).collect();
        assert_eq!(names, ["000", "aaa"]);
    }

    #[test]
    fn builds_flag_summaries() {
        let command = Command::with_name("")
            .with_flag(Flag::with_name("aaa"))
            .with_flag(Flag::with_name("bbb").with_alias("b"))
            .with_flag(Flag::with_name("ccc").with_alias("c").accept_value())
            .with_flag(Flag::with_name("ddd").with_alias("d"))
            .with_flag(Flag::with_name("eee"));
        let args = vec!["--aaa".to_string(), "-c".to_string(), "cval".to_string(), "--eee".to_string()];
        let summaries = build_flag_summaries(&command, &args).unwrap();
        let total = summaries.len();
        let provided: Vec<FlagSummary> = summaries.into_iter()
            .filter(|s| s.provided()).collect();
        let names: Vec<String> = provided.iter()
            .map(|s| s.name().clone()).collect();
        assert_eq!(total, 5);
        assert_eq!(names, ["aaa", "ccc", "eee"]);
        assert_eq!(provided.get(1).unwrap().value().as_ref().unwrap(), "cval");
    }
}
