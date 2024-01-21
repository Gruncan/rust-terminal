use std::process;

use crate::terminal::Terminal;

pub(crate) struct CommandWrapper {
    name: String,
    help: String,
    exec: Box<dyn CommandExecutor>,

}

impl CommandWrapper {
    pub(crate) fn new(name: &str, help: &str, exec: Box<dyn CommandExecutor>) -> CommandWrapper {
        CommandWrapper { name: String::from(name), help: String::from(help), exec }
    }

    pub(crate) fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        self.exec.execute(cmd_string_line.replace(&self.name, ""), terminal)
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
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal);
}

impl CommandExecutor for ChangeDir {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In change dir!");
    }
}

impl CommandExecutor for GetPath {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In get path!");
    }
}

impl CommandExecutor for SetPath {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In set path!");
    }
}

impl CommandExecutor for History {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In history")
    }
}

impl CommandExecutor for LastCommand {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In last command")
    }
}

impl CommandExecutor for NCommand {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In n command")
    }
}

impl CommandExecutor for NMinusCommand {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In n minus command")
    }
}

impl CommandExecutor for Alias {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In alias")
    }
}

impl CommandExecutor for UnAlias {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In unalias")
    }
}

impl CommandExecutor for Exit {
    fn execute(&self, cmd_string_line: String, terminal: &mut Terminal) {
        println!("In exit!");
        process::exit(0);
    }
}
