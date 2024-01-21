use std;
use std::collections::HashMap;

use crate::command::Command;

pub(crate) struct Terminal {
    pub working_dir: String,
    pub system_path: String,
    history: Vec<(Command, String)>,
    aliases: HashMap<String, (Command, Option<String>)>,

}

impl Terminal {
    pub fn new(home_dir: String, system_dir: String) -> Terminal {
        Terminal {
            working_dir: home_dir,
            system_path: system_dir,
            history: Vec::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn append(&mut self, command: Command, cmd_string_line: &str) {
        self.history.push((command, String::from(cmd_string_line)));
    }

    pub fn get_history_commands(&self) -> Vec<String> {
        // TODO history should maybe include itself
        self.history.iter().map(|item| String::from(item.1.as_str())).collect()
    }

    pub fn get_aliases(&self) -> Vec<String> {
        self.aliases.iter()
            .map(|(key, (cmd, args))| format!("\"{}\" => {} {}", key, cmd.to_string(),
                                              args.unwrap_or(String::from(""))
                                                  .collect()
            }

    pub fn add_alias(&mut self, name: String, command: Command, args: Option<String>) {
        self.aliases.insert(name, (command, args));
    }

    pub fn is_alias_present(&self, alias: &String) -> bool {
        self.aliases.contains_key(alias)
    }

    pub fn run_prev_command(&mut self, mut i: i32) -> bool {
        if self.history.is_empty() {
            println!("You have not ran a command yet.");
            return true;
        }
        let size: i32 = self.history.len() as i32;
        if i == 0 {
            println!("There is no zero in the history silly.");
            return true;
        }
        let temp_option = if i < 0 {
            println!("DEBUG: {}", size + i);
            self.history.get((size + i) as usize)
        } else {
            i -= 1;
            self.history.get(i as usize)
        };
        return if let Some(cmd_option) = temp_option {
            let (command, user_input) = cmd_option.clone();
            Terminal::run_command(Some((Command::from(command), String::from(user_input))), self)
        } else {
            println!("Invalid index passed.");
            false
        };
    }

    pub fn run_command(cmd_option: Option<(Command, String)>, terminal: &mut Terminal) -> bool {
        if let Some((command, user_input)) = cmd_option {
            let command_wrapper = command.get_command();
            let suc = command_wrapper.execute(user_input.as_str(), terminal);
            if suc && !command_wrapper.name.starts_with("!") {
                terminal.append(command, user_input.as_str());
            }
            return suc;
        } else {
            eprintln!("Unknown command, try again.")
        }
        return false;
    }

}