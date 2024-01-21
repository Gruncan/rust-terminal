// 1. Execution of external system programs including their parameters.
// 2. The following built-in commands:
// a. cd – change working directory
// b. getpath – print system path
// c. setpath – set system path
// d. history – print history contents (numbered list of commands in history
// including their parameters in ascending order from least to most recent)
// e. !! – invoke the last command from history (e.g. if your last command
// was ‘ls –lF’ then ‘!!’ should execute ‘ls –lF’)
// f. !<no> - invoke command with number <no> from history (e.g. !5 will
// execute the command with number 5 from the history)
// g. !-<no> - invoke the command with number the number of the current
// command minus <no> (e.g. !-3 if the current command number is 5 will
// execute the command with number 2, or !-1 will execute again the last
// command)
// h. alias – print all set aliases (alias plus aliased command)
// i. alias <name> <command> - alias name to be the command. Note that
// the command may also include any number of parameters, while (any
// number of) command parameters should work correctly with aliasing
// (e.g. if I alias la to be ls –la then when I type la . the shell should execute
// ls –la .). Note also that aliasing should also work correctly with history
// (e.g. !5 will execute the command with number 5 from the history if this
// command is an alias like the la above then !5 will execute ls –la). In the
// enhanced form of the alias, it should be possible to alias history
// invocations (e.g. if I alias ‘five’ to be !5 then when I type ‘five’ the shell
// should execute the command with number 5 from the history). It should
// also be possible to alias aliases (e.g. If I alias l to be ls and then I alias la
// to be l –a, then when I type la the shell should execute ls –a).
// j. unalias <command> - remove any associated alias
// 3. Persistent history of user commands (save history in a file and load it when you run
// the shell again)
// 4. Persistent aliases (save aliases in a file and load it when you run the shell again)

use std::env;
use std::io;
use std::io::Write;

use crate::command::Command;
use crate::terminal::Terminal;

mod command;
mod terminal;

fn main() {
    let cwd = get_current_working_directory();
    let hd = get_current_home_directory();
    let mut terminal: Terminal;

    if hd.is_some() && cwd.is_some() {
        terminal = Terminal::new(hd.unwrap(), cwd.unwrap());
    } else {
        eprintln!("Failed to initialize a terminal instance!");
        eprintln!("Home or working directory not set.");
        return;
    }

    startup_util();


    loop {
        let mut user_input = String::new();
        print!("({}) > ", terminal.working_dir);
        io::stdout().flush().expect("Failed to flush output");
        io::stdin().read_line(&mut user_input).expect("Failed to read user input");
        let cmd_option: Option<(Command, String)> = parse_user_input(user_input);
        Terminal::run_command(cmd_option, &mut terminal);
    }
}

// TODO move into terminal struct to handle aliases
fn parse_user_input(user_input: String) -> Option<(Command, String)> {
    let split_user_input: Vec<String> = user_input.split(" ").map(|v| String::from(v)).collect();
    if let Some(string_command) = split_user_input.first() {
        // Correctly handle command if statement order matters
        let cmd_option: Option<Command> = if string_command.starts_with("!!") {
            Command::get_command_enum("!!")
        } else if string_command.starts_with("!-") {
            Command::get_command_enum("!-")
        } else if string_command.starts_with("!") {
            Command::get_command_enum("!")
        } else {
            Command::get_command_enum(string_command.trim())
        };
        return if let Some(command) = cmd_option {
            Some((command, user_input))
        } else {
            None
        };
    }

    return None;
}


fn startup_util() {
    // TODO load history
    // TODO load aliases
}

fn get_current_working_directory() -> Option<String> {
    return match env::current_dir() {
        Ok(work_dir) => {
            let cwd = work_dir.to_str();
            if cwd.is_some() {
                Some(String::from(cwd.unwrap()))
            } else {
                None
            }
        }
        Err(_) => {
            None
        }
    };
}

fn get_current_home_directory() -> Option<String> {
    // TODO if HOME is not found check USERPROFILE for win, HOMEPATH for linux
    return match env::var("USERPROFILE") {
        Ok(home_dir) => Some(home_dir),
        Err(_) => None
    };
}
