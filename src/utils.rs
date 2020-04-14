use crate::{Command, CommandSummary, FlagSummary};

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

/// Returns summary objects of parent commands. 
pub fn build_supcommand_summaries(app: &Command, positions: &Vec<usize>) -> Vec<CommandSummary> {
    let mut list = Vec::new();
    for _ in positions.into_iter() {
        let command = command_at_position(&app, &positions);
        list.push(command.summarize());
    }
    list
}

/// Returns summary objects of child commands. 
pub fn build_subcommand_summaries(command: &Command) -> Vec<CommandSummary> {
    let mut list = Vec::new();
    for subcommand in command.commands().into_iter() {
        list.push(subcommand.summarize());
    }
    list
}

/// Returns flag summary objects for command. 
pub fn summarizebuild_flag_summaries(command: &Command) -> Vec<FlagSummary> {
    let mut list = Vec::new();
    for flag in command.flags().into_iter() {
        list.push(flag.summarize());
    }
    list
}
