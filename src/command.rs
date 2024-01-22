use std::fs::metadata;
use std::process;

use crate::terminal::Terminal;

pub(crate) struct CommandWrapper {
    pub name: String,
    help: String,
    exec: Box<dyn CommandExecutor>,

}

impl CommandWrapper {
    pub(crate) fn new(name: &str, help: &str, exec: Box<dyn CommandExecutor>) -> CommandWrapper {
        CommandWrapper { name: String::from(name), help: String::from(help), exec }
    }

    pub(crate) fn execute(&self, cmd_string_line: &str, terminal: &mut Terminal) -> bool {
        let parsed_cmd_line = cmd_string_line.replace(&self.name, "");
        let parsed_cmd_line = String::from(parsed_cmd_line.trim());
        let parsed_cmd_option: Option<String>;
        if parsed_cmd_line.is_empty() {
            parsed_cmd_option = None;
        } else {
            parsed_cmd_option = Some(parsed_cmd_line);
        }
        self.exec.execute(parsed_cmd_option, terminal)
    }
}


pub(crate) enum Command {
    ChangeDir,
    GetPath,
    SetPath,
    History,
    LastCommand,
    NCommand,
    NMinusCommand,
    Alias,
    Unalias,
    // Alternative to closing
    Exit,
}

impl PartialEq<Self> for Command {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Eq for Command {}

impl Command {
    fn change_dir_command() -> CommandWrapper {
        CommandWrapper::new("cd",
                            "change working directory",
                            Box::new(ChangeDir))
    }

    fn get_path_command() -> CommandWrapper {
        CommandWrapper::new("getpath",
                            "print system path",
                            Box::new(GetPath))
    }

    fn set_path_command() -> CommandWrapper {
        CommandWrapper::new("setpath",
                            "set system path",
                            Box::new(SetPath))
    }

    fn history_command() -> CommandWrapper {
        CommandWrapper::new("history",
                            "print history contents",
                            Box::new(History))
    }

    fn last_command() -> CommandWrapper {
        CommandWrapper::new("!!",
                            "invoke the last command from history",
                            Box::new(LastCommand))
    }

    fn n_command() -> CommandWrapper {
        CommandWrapper::new("!",
                            "invoke command with number",
                            Box::new(NCommand))
    }

    fn n_minus_command() -> CommandWrapper {
        CommandWrapper::new("!-",
                            "invoke the command with number the number of the currentcommand minus ",
                            Box::new(NMinusCommand))
    }

    fn alias_command() -> CommandWrapper {
        CommandWrapper::new("alias",
                            "print all set aliases | alias name to be the command",
                            Box::new(Alias))
    }

    fn unalias_command() -> CommandWrapper {
        CommandWrapper::new("unalias",
                            "remove any associated alias",
                            Box::new(UnAlias))
    }

    fn exit_command() -> CommandWrapper {
        CommandWrapper::new("exit",
                            "close down terminal",
                            Box::new(Exit))
    }

    pub fn get_command(&self) -> CommandWrapper {
        return match self {
            Command::ChangeDir => Command::change_dir_command(),
            Command::GetPath => Command::get_path_command(),
            Command::SetPath => Command::set_path_command(),
            Command::History => Command::history_command(),
            Command::LastCommand => Command::last_command(),
            Command::NCommand => Command::n_command(),
            Command::NMinusCommand => Command::n_minus_command(),
            Command::Alias => Command::alias_command(),
            Command::Unalias => Command::unalias_command(),
            Command::Exit => Command::exit_command(),
        };
    }

    pub fn get_command_enum(cmd_string: &str) -> Option<Command> {
        return match cmd_string {
            "cd" => Some(Command::ChangeDir),
            "getpath" => Some(Command::GetPath),
            "setpath" => Some(Command::SetPath),
            "history" => Some(Command::History),
            "!!" => Some(Command::LastCommand),
            "!" => Some(Command::NCommand),
            "!-" => Some(Command::NMinusCommand),
            "alias" => Some(Command::Alias),
            "unalias" => Some(Command::Unalias),
            "exit" => Some(Command::Exit),
            _ => None,
        };
    }

    pub fn from(c: &Command) -> Command {
        match *c {
            Command::ChangeDir => Command::ChangeDir,
            Command::GetPath => Command::GetPath,
            Command::SetPath => Command::SetPath,
            Command::History => Command::History,
            Command::LastCommand => Command::LastCommand,
            Command::NCommand => Command::NCommand,
            Command::NMinusCommand => Command::NMinusCommand,
            Command::Alias => Command::Alias,
            Command::Unalias => Command::Unalias,
            Command::Exit => Command::Exit,
        }
    }

    pub fn to_string(&self) -> String {
        self.get_command().name
    }
}


struct ChangeDir;

struct GetPath;

struct SetPath;

struct History;

struct LastCommand;

struct NCommand;

struct NMinusCommand;

struct Alias;

struct UnAlias;

struct Exit;

pub(crate) trait CommandExecutor {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool;
}

fn is_valid_directory(dir: &str) -> bool {
    metadata(dir).map(|metadata| metadata.is_dir()).unwrap_or(false)
}

impl CommandExecutor for ChangeDir {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        // TODO make directory handling relative to current path
        if let Some(directory) = cmd_string_line {
            if is_valid_directory(directory.as_str()) {
                terminal.working_dir = directory;
                return true;
            } else {
                println!("Directory not valid!")
            }
        } else {
            println!("Please input a path to change to.");
        }
        return false;
    }
}

impl CommandExecutor for GetPath {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        println!("{}", terminal.system_path);
        return true;
    }
}

impl CommandExecutor for SetPath {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        if let Some(directory) = cmd_string_line {
            if is_valid_directory(directory.as_str()) {
                terminal.system_path = directory;
                return true;
            } else {
                println!("Directory not valid!")
            }
        } else {
            println!("Please a path to set.")
        }
        return false;
    }
}

impl CommandExecutor for History {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        let commands = terminal.get_history_commands();
        if commands.is_empty() {
            println!("There is no history!");
        } else {
            for (index, command) in commands.iter().enumerate() {
                println!("{}. {}", index + 1, command.trim());
            }
        }
        return true;
    }
}

impl CommandExecutor for LastCommand {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        terminal.run_prev_command(1)
    }
}

fn nx_command_executor(cmd_string_line: Option<String>, terminal: &mut Terminal, go_back: bool) -> bool {
    let mut multiply: i32 = 1;
    if go_back {
        multiply = -1;
    }
    return if let Some(number_str) = cmd_string_line {
        return if let Ok(num) = number_str.parse::<i32>() {
            terminal.run_prev_command(num * multiply)
        } else {
            println!("Failed to parse number!");
            false
        }
    } else {
        println!("Nothing to parse.");
        false
    }
}


impl CommandExecutor for NCommand {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        return nx_command_executor(cmd_string_line, terminal, false);
    }
}

impl CommandExecutor for NMinusCommand {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        return nx_command_executor(cmd_string_line, terminal, true);
    }
}

impl CommandExecutor for Alias {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        if cmd_string_line.is_none() {
            let aliases = terminal.get_aliases_string();
            if aliases.is_empty() {
                println!("No aliases set!");
            } else {
                for alias in terminal.get_aliases_string() {
                    println!("{}", alias);
                }
            }
            return true
        } else {
            let cmd_string_line = cmd_string_line.unwrap();
            let alias_line: Vec<String> = cmd_string_line.splitn(3, " ").map(|v| String::from(v)).collect();
            if alias_line.len() >= 2 {
                let name = alias_line.get(0).unwrap();
                let command = alias_line.get(1).unwrap();
                let command_option = Command::get_command_enum(command.as_str());
                if command_option.is_none() {
                    println!("Unknown command \"{}\" trying to be set as an alias", command);
                    return false;
                } else if terminal.is_alias_present(name) {
                    println!("This alias is already been set.");
                }

                if alias_line.len() == 2 { // no args
                    terminal.add_alias(String::from(name), command_option.unwrap(), None);
                } else { // Args
                    let arguments = alias_line.get(2).unwrap();
                    terminal.add_alias(String::from(name), command_option.unwrap(), Some(String::from(arguments)))
                }
                return true;
            } else {
                println!("Set aliases with: > alias <name> <command>")
            }
        }
        return false;
    }
}

impl CommandExecutor for UnAlias {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        return if cmd_string_line.is_none() {
            println!("Unset an alias with: > unalias <command> ");
            false
        } else {
            let cmd_string_line = cmd_string_line.unwrap();
            let command_option: Option<(Command, String)> = terminal.parse_user_input(cmd_string_line);
            return if let Some((command, _)) = command_option {
                let aliases_removed: Vec<String> = terminal.remove_command_aliases(command);
                if aliases_removed.is_empty() {
                    println!("There are no aliases associated with that command.")
                } else {
                    println!("Removed the following aliases:");
                    for alias in &aliases_removed {
                        println!(" - \"{}\"", alias)
                    }
                }
                true
            } else {
                println!("That is not a valid command!");
                false
            };
        };
    }
}

impl CommandExecutor for Exit {
    fn execute(&self, cmd_string_line: Option<String>, terminal: &mut Terminal) -> bool {
        println!("Goodbye!");
        process::exit(0);
    }
}
