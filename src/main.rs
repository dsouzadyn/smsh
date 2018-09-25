use std::io::{self, Write};
use std::process::Command;
use std::collections::HashMap;

#[derive(PartialEq)]
struct ShellCommand {
    name:&'static str,
    command_type: CommandType,
}

#[derive(PartialEq)]
enum CommandType {
    INBUILT = 0,
    CUSTOM = 1,
}

fn flush(stdout: &mut io::Stdout) {
    stdout.flush().expect("Error: could not flush buffer!");
}

impl ShellCommand {
    fn new(name: &'static str, command_type: CommandType) -> ShellCommand {
        ShellCommand { name: name, command_type: command_type}
    }

    fn execute(&self, args: Vec<&str>) {
        let child = Command::new(&self.name).args(args.iter()).spawn().expect("could not spawn process");
        let output = child.wait_with_output().expect("Oops something went wrong");

        print!("{}", String::from_utf8_lossy(&output.stdout));
        print!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn exit() {
    std::process::exit(0x0000);
}

fn process(buf: String, stdout: &mut io::Stdout, commands: &HashMap<&str, ShellCommand>) {
    let tokens: Vec<&str> = buf.trim()
        .split(' ').collect();
    let command = commands.get(tokens[0]);

    if command.is_none() != true {
        let comm = command.unwrap();
        match comm.command_type {
            CommandType::INBUILT => comm.execute(tokens[1..].to_vec()),
            CommandType::CUSTOM => {
                if comm.name == "exit" { exit() };
            },
        }
    } else {
        print!("Error: No such command exists!\n");
    }

    flush(stdout);
}

fn shell_loop() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut shell_commands: HashMap<&str, ShellCommand> = HashMap::new();

    shell_commands.insert("clear", ShellCommand::new("clear", CommandType::INBUILT));
    shell_commands.insert("ls", ShellCommand::new("ls", CommandType::INBUILT));
    shell_commands.insert("exit", ShellCommand::new("exit", CommandType::CUSTOM));

    loop {
        let mut buffer = String::new();
        stdout.write(b"><>>> ").expect("Error: could not write to stdout!");
        flush(&mut stdout);
        stdin.read_line(&mut buffer).expect("Error: could not read into buffer!");
        process(buffer, &mut stdout, &shell_commands);
    }
}

fn main() {
    shell_loop();
}
