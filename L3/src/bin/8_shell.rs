use nix::sys::signal::{kill as nix_kill, Signal};
use nix::sys::wait::wait;
use nix::unistd::Pid;
use nix::unistd::{fork, ForkResult};
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::{self, Command, Stdio};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let prompt = "my_shell> ";

    loop {
        print!("{}", prompt);
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin
            .lock()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // Exit command
        if input == "\\quit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => {
                if let Some(dir) = args.get(0) {
                    if let Err(err) = env::set_current_dir(dir) {
                        eprintln!("cd: {}: {}", dir, err);
                    }
                } else {
                    eprintln!("cd: missing argument");
                }
            }
            "pwd" => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(err) => eprintln!("pwd: {}", err),
            },
            "echo" => {
                println!("{}", args.join(" "));
            }
            "kill" => {
                if let Some(pid_str) = args.get(0) {
                    if let Ok(pid) = pid_str.parse::<i32>() {
                        if let Err(err) = nix_kill(Pid::from_raw(pid), Signal::SIGKILL) {
                            eprintln!("kill: failed to kill process {}: {}", pid, err);
                        }
                    } else {
                        eprintln!("kill: invalid PID: {}", pid_str);
                    }
                } else {
                    eprintln!("kill: missing argument");
                }
            }
            "ps" => match fs::read_dir("/proc") {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Ok(pid) = entry.file_name().to_string_lossy().parse::<i32>() {
                                let cmdline_path = format!("/proc/{}/cmdline", pid);
                                let cmdline = fs::read_to_string(cmdline_path)
                                    .unwrap_or("unknown".to_string());
                                let uptime_path = format!("/proc/{}/stat", pid);
                                let uptime_info =
                                    fs::read_to_string(uptime_path).unwrap_or("0".to_string());
                                let uptime: Vec<&str> = uptime_info.split_whitespace().collect();
                                if uptime.len() > 21 {
                                    let start_time = uptime[21].parse::<i64>().unwrap_or(0);
                                    println!(
                                        "PID: {}, CMD: {}, START_TIME: {}",
                                        pid, cmdline, start_time
                                    );
                                }
                            }
                        }
                    }
                }
                Err(err) => eprintln!("ps: failed to read /proc: {}", err),
            },
            _ => {
                if input.contains('|') {
                    let commands: Vec<&str> = input.split('|').collect();
                    if let Err(err) = execute_pipe(commands) {
                        eprintln!("Error executing pipe: {}", err);
                    }
                } else {
                    // Запуск через fork/exec
                    if let Err(err) = execute_command(command, &args) {
                        eprintln!("Error executing command: {}", err);
                    }
                }
            }
        }
    }
}

fn execute_command(command: &str, args: &[&str]) -> io::Result<()> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child: _ }) => {
            wait().expect("waitpid failed");
        }
        Ok(ForkResult::Child) => {
            let cmd = Command::new(command).args(args).spawn();
            if let Err(err) = cmd {
                eprintln!("exec error: {}", err);
            }
            process::exit(0);
        }
        Err(err) => {
            eprintln!("fork failed: {}", err);
        }
    }
    Ok(())
}

fn execute_pipe(commands: Vec<&str>) -> io::Result<()> {
    let mut prev_cmd: Option<Command> = None;

    for (i, command) in commands.iter().enumerate() {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        if i == commands.len() - 1 {
            let mut final_cmd = if let Some(ref mut prev_cmd) = prev_cmd.take() {
                prev_cmd
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()?
            } else {
                Command::new(cmd).args(&args).spawn()?
            };
            final_cmd.wait()?;
        } else {
            let pipe_cmd = if let Some(ref mut prev_cmd) = prev_cmd.take() {
                prev_cmd
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()?
            } else {
                Command::new(cmd)
                    .args(&args)
                    .stdout(Stdio::piped())
                    .spawn()?
            };
            let mut next_commnad = Command::new(cmd);
            next_commnad
                .args(&args)
                .stdin(Stdio::from(pipe_cmd.stdout.unwrap()));
            prev_cmd = Some(next_commnad);
        }
    }

    Ok(())
}
