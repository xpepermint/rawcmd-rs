use crate::{Command, CommandSummary, Flag, FlagSummary};

/// Parses arguments and finds command positions in a tree.
pub fn build_command_positions(app: &Command, args: &Vec<String>) -> Vec<usize> {
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
        for (index, cmd) in commands.into_iter().enumerate() {
            command = cmd;
            if *cmd.name() == arg {
                positions.push(index);
                break;
            } else if index == size - 1 {
                panic!("invalid command");
            }
        }
    }

    positions
}

/// Returns command object based on the position in arguments.
pub fn command_at_position<'a>(app: &'a Command, positions: &Vec<usize>) -> &'a Command {
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
        command.description().clone(),
        command.hint().clone(),
        command.author().clone(),
        command.version().clone(),
    )
}

/// Returns command summary.
pub fn build_flag_summary(flag: &Flag, provided: bool, value: &Option<String>) -> FlagSummary {
    FlagSummary::with_name(
        flag.name().clone().as_str(),
        flag.alias().clone(),
        flag.hint().clone(),
        value.clone(),
        flag.default_value().clone(),
        flag.accepts_value().clone(),
        provided.clone(),
    )
}

/// Returns summary objects of parent commands. 
pub fn build_supcommand_summaries(app: &Command, positions: &Vec<usize>) -> Vec<CommandSummary> {
    let mut all = Vec::new();
    for _ in positions.into_iter() {
        let command = command_at_position(&app, &positions);
        all.push(build_command_summary(command));
    }
    all
}

/// Returns summary objects of child commands. 
pub fn build_subcommand_summaries(command: &Command) -> Vec<CommandSummary> {
    let mut all = Vec::new();
    for subcommand in command.commands().into_iter() {
        all.push(build_command_summary(subcommand));
    }
    all
}

/// Returns flag summary objects for command. 
pub fn build_flag_summaries(command: &Command, args: &Vec<String>) -> Vec<FlagSummary> {
    let mut all = Vec::new();

    for (index, arg) in args.into_iter().enumerate() {

        if !arg.starts_with("-") {
            continue
        } else if arg == "--" {
            break;
        }

        let flag = command.flags().iter().find(|&f| {
            *arg == format!("{}{}", "--", &f.name())
            || f.alias().is_some() && *arg == format!("{}{}", "-", f.alias().as_ref().unwrap())
        });
        let flag = match flag {
            Some(f) => f,
            None => panic!("unknown flag"),
        };

        let value = if flag.accepts_value() {
            Some(match &args.get(index + 1) {
                Some(v) => v.to_string(),
                None => panic!("flag has no value"),
            })
        } else {
            None
        };
        all.push(build_flag_summary(&flag, true, &value));
    }
 
    for flag in command.flags().into_iter() {
        let exists = &all.iter().any(|f| f.name() == flag.name());
        if !exists {
            all.push(build_flag_summary(flag, false, &None));
        }
    }

    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_flag_summaries() {
        let command = Command::with_name("")
            .with_flag(Flag::with_name("aaa"))
            .with_flag(Flag::with_name("bbb").with_alias("b"))
            .with_flag(Flag::with_name("ccc").with_alias("c").accept_value())
            .with_flag(Flag::with_name("ddd").with_alias("d"))
            .with_flag(Flag::with_name("eee"));
        let args = vec!["--aaa".to_string(), "-c".to_string(), "cval".to_string(), "--eee".to_string()];
        let summaries = build_flag_summaries(&command, &args);
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
