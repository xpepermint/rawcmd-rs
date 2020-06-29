use std::env;
use crate::{Result, Error, ErrorKind, Command, CommandSummary, Flag, FlagSummary,
    Param, ParamSummary, Resource, ResourceSummary};

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
pub fn build_subcommand_positions<C, A, T>(app: &Command<C>, args: A) -> Result<Vec<usize>>
    where
    A: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut args: Vec<String> = args.into_iter().map(Into::into).collect();
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
pub fn subcommand_at_position<'a, C>(app: &'a Command<C>, positions: &Vec<usize>) -> &'a Command<C> {
    let mut command = app;
    for position in positions.clone().into_iter() {
        command = &command.commands().get(position).unwrap();
    }
    command
}

/// Returns command summary.
pub fn build_command_summary<C>(command: &Command<C>) -> CommandSummary {
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

/// Returns command summary.
pub fn build_param_summary(param: &Param, provided: bool, value: &Option<String>) -> ParamSummary {
    ParamSummary::with_name(
        param.name().clone().as_str(),
        param.description().clone(),
        value.clone(),
        param.default_value().clone(),
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
pub fn build_supcommand_summaries<C>(app: &Command<C>, positions: &Vec<usize>) -> Vec<CommandSummary> {
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
pub fn build_subcommand_summaries<C>(command: &Command<C>) -> Vec<CommandSummary> {
    let mut items = Vec::new();
    for subcommand in command.commands().into_iter() {
        items.push(build_command_summary(subcommand));
    }
    items.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));
    items
}

/// Returns flag summary objects for command. 
pub fn build_flag_summaries<C, A, T>(command: &Command<C>, args: A) -> Result<Vec<FlagSummary>>
    where
    A: IntoIterator<Item = T>,
    T: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    let mut items = Vec::new();
    for (index, arg) in args.iter().enumerate() {

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
    
        items.push(build_flag_summary(flag, true, &value));
    }
 
    for flag in command.flags().into_iter() {
        let exists = &items.iter().any(|f| f.name() == flag.name());
        if !exists {
            let value = match flag.default_value() {
                Some(v) => Some(v.to_string()),
                None => None,
            };
            items.push(build_flag_summary(flag, false, &value));
        }
    }
    items.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));
    Ok(items)
}

/// Returns param summary objects for command. 
pub fn build_param_summaries<C, A, T>(command: &Command<C>, args: A) -> Result<Vec<ParamSummary>>
    where
    A: IntoIterator<Item = T>,
    T: Into<String>,
{
    let mut args = args.into_iter().map(Into::into).collect::<Vec<String>>();
    args.reverse();

    let mut inputs = Vec::new();
    let mut command = command;
    let mut stage = 0; // 0..comman, 1..flag, 2..param
    while let Some(arg) = args.pop() {
        if arg == "--" {
            break;
        } else if arg.starts_with("-") {
            if let Some(flag) = command.flags().iter().find(|f| {
                arg.starts_with("--") && f.name().eq(&arg[2..].to_string())
                || arg.starts_with("-") && f.alias().eq(&Some(arg[1..].to_string()))
            }) {
                if flag.accepts_value() {
                    args.pop();
                }
            }
            stage = 1;
            continue;
        } else if stage == 1 && !arg.starts_with("-") {
            stage = 2;
        } else if stage == 0 {
            if let Some(subcmd) = command.commands().iter().find(|c| c.name().eq(&arg)) {
                command = subcmd;
                continue;
            } else {
                stage = 2;
            }
        }
        if stage == 2 {
            inputs.push(arg);
        }
    }

    let mut params = command.params().clone();
    params.reverse();

    let params_count = params.len();
    let input_count = inputs.len();
    if params_count < input_count {
        return Err(Error::new(ErrorKind::ToManyParams(params_count, input_count)));
    }

    let mut items = Vec::new();
    for (index, param) in params.iter().enumerate() {
        let input = match inputs.get(index) {
            Some(input) => Some(input.to_string()),
            None => None,
        };
        items.push(build_param_summary(param, input.is_some(), &input));
    }
    items.reverse();
    
    Ok(items)
}

/// Returns resource summary objects for command. 
pub fn build_resource_summaries<C>(command: &Command<C>) -> Vec<ResourceSummary> {
    command.resources().iter().map(|r| {
        build_resource_summary(r)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Context;

    #[test]
    fn splits_equal_args() {
        assert_eq!(
            split_equal_args(&vec!["aa", "bb=11", "--cc=22", "-d=33"].iter().map(|s| s.to_string()).collect()),
            vec!["aa", "bb", "11", "--cc", "22", "-d", "33"],
        );
    }

    #[test]
    fn builds_command_positions() {
        let command = Command::<Context>::with_name("000")
            .with_subcommand(
                Command::with_name("aaa")
                    .with_subcommand(Command::with_name("bbb"))
                    .with_subcommand(Command::with_name("ccc"))
        );
        let positions = build_subcommand_positions(&command, vec!["aaa", "ccc"]).unwrap();
        let total = positions.len();
        assert_eq!(total, 2);
        assert_eq!(positions, [0, 1]);
    }

    #[test]
    fn builds_supcommand_summaries() {
        let command = Command::<Context>::with_name("000")
            .with_subcommand(
                Command::with_name("aaa")
                    .with_subcommand(
                        Command::with_name("bbb")
                    )
            );
        let positions = build_subcommand_positions(&command, vec!["aaa", "bbb"]).unwrap();
        let summaries = build_supcommand_summaries(&command, &positions);
        let names: Vec<String> = summaries.iter()
            .map(|s| s.name().clone()).collect();
        assert_eq!(names, ["000", "aaa"]);
    }

    #[test]
    fn builds_flag_summaries() {
        let command = Command::<Context>::with_name("")
            .with_flag(Flag::with_name("aaa"))
            .with_flag(Flag::with_name("bbb").with_alias("b"))
            .with_flag(Flag::with_name("ccc").with_alias("c").accept_value())
            .with_flag(Flag::with_name("ddd").with_alias("d"))
            .with_flag(Flag::with_name("eee"));
        let summaries0 = build_flag_summaries(&command, vec!["cmd", "--aaa", "-c", "cval", "--eee", "--"]).unwrap();
        let summaries1 = build_flag_summaries(&command, vec!["--aaa", "-c", "cval", "--eee"]).unwrap();
        let provided0: Vec<FlagSummary> = summaries0.iter().filter(|s| s.provided()).cloned().collect();
        let provided1: Vec<FlagSummary> = summaries1.iter().filter(|s| s.provided()).cloned().collect();
        let names0: Vec<String> = provided0.iter().map(|s| s.name().clone()).collect();
        let names1: Vec<String> = provided1.iter().map(|s| s.name().clone()).collect();
        assert_eq!(summaries0.len(), 5);
        assert_eq!(summaries1.len(), 5);
        assert_eq!(names0, ["aaa", "ccc", "eee"]);
        assert_eq!(names1, ["aaa", "ccc", "eee"]);
        assert_eq!(provided0.get(1).unwrap().value().as_ref().unwrap(), "cval");
    }

    #[test]
    fn builds_param_summaries() {
        let command = Command::<Context>::with_name("")
            .with_subcommand(
                Command::with_name("cmd")
                    .with_flag(Flag::with_name("bbb").with_alias("c"))
                    .with_param(Param::with_name("ddd"))
            )
            .with_flag(Flag::with_name("aaa"))
            .with_flag(Flag::with_name("bbb").with_alias("c").accept_value())
            .with_param(Param::with_name("aaa"))
            .with_param(Param::with_name("bbb"))
            .with_param(Param::with_name("ccc"));
        let summaries0 = build_param_summaries(&command, vec!["--aaa", "-c", "x", "bbb", "ccc", "--"]).unwrap();
        let summaries1 = build_param_summaries(&command, vec!["cmd", "-c", "ddd"]).unwrap();
        let summaries2 = build_param_summaries(&command, vec!["cmd", "ddd"]).unwrap();
        let provided0: Vec<ParamSummary> = summaries0.iter().filter(|s| s.provided()).cloned().collect();
        let provided1: Vec<ParamSummary> = summaries1.iter().filter(|s| s.provided()).cloned().collect();
        let provided2: Vec<ParamSummary> = summaries2.iter().filter(|s| s.provided()).cloned().collect();
        let names0: Vec<String> = provided0.iter().map(|s| s.name().clone()).collect();
        let names1: Vec<String> = provided1.iter().map(|s| s.name().clone()).collect();
        let names2: Vec<String> = provided2.iter().map(|s| s.name().clone()).collect();
        assert_eq!(summaries0.len(), 3);
        assert_eq!(summaries1.len(), 1);
        assert_eq!(summaries2.len(), 1);
        assert_eq!(names0, ["bbb", "ccc"]);
        assert_eq!(names1, ["ddd"]);
        assert_eq!(names2, ["ddd"]);
    }
}
