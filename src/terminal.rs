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


    pub fn get_aliases_string(&self) -> Vec<String> {
        self.aliases.iter()
            .map(|(key, (cmd, args))| get_format(String::from(key), cmd.to_string(), args))
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
            self.history.get((size + i) as usize)
        } else {
            i -= 1;
            self.history.get(i as usize)
        };
        return if let Some(cmd_option) = temp_option {
            let (command, user_input) = cmd_option.clone();
            self.run_command(Some((Command::from(command), String::from(user_input))))
        } else {
            println!("Invalid index passed.");
            false
        };
    }

    // TODO command is not in history from alias
    pub fn run_command(&mut self, cmd_option: Option<(Command, String)>) -> bool {
        if let Some((command, user_input)) = cmd_option {
            let command_wrapper = command.get_command();
            let suc = command_wrapper.execute(user_input.as_str(), self);
            if suc && !command_wrapper.name.starts_with("!") {
                self.append(command, user_input.as_str());
            }
            return suc;
        } else {
            eprintln!("Unknown command, try again.")
        }
        return false;
    }

    pub fn parse_user_input(&mut self, user_input: String) -> Option<(Command, String)> {
        let split_user_input: Vec<String> = user_input.trim().split(" ").map(|v| String::from(v)).collect();
        if let Some(mut string_command) = split_user_input.first() {
            let option: Option<&(Command, Option<String>)> = self.aliases.get(string_command);
            if let Some((cmd, args)) = option {
                return if let Some(args_string) = args {
                    Some((Command::from(cmd), format!("{} {}", cmd.to_string(), args_string)))
                } else {
                    Some((Command::from(cmd), cmd.to_string()))
                }
            }

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

    pub fn remove_command_aliases(&mut self, command: Command) -> Vec<String> {
        let aliases_to_remove: Vec<String> = self.aliases.iter()
            .filter(|(_, (cmd, _))| *cmd == command)
            .map(|(key, (_, _))| key)
            .cloned()
            .collect();
        for alias in &aliases_to_remove {
            self.aliases.remove(alias);
        }

        aliases_to_remove
    }

}

fn get_format(key: String, cmd: String, args: &Option<String>) -> String {
    format!("\"{}\" => {} {}", key, cmd.to_string(), args.clone().unwrap_or(String::from("")))
}