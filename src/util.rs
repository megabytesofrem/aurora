use expanduser::expanduser;
use std::process::Command;

// Helper function to check whether a package is installed or not
// using the UNIX which utility.
pub fn is_installed(prog: &str) -> bool {
    let out = run_command("which", vec![prog.into()]);
    return out.len() > 0;
}

pub fn expand_path(path: &str) -> String {
    let expanded = expanduser(path);
    expanded.unwrap().display().to_string()
    //path.unwrap().display().to_string()
}

pub fn run_command_cd(cmd: &str, args: Vec<String>, cd: &str) -> String {
    let output = Command::new(cmd)
                        .current_dir(cd)
                        .args(args)
                        .status()
                        .expect("failed to execute command");

    // print any errors
    format!("{:?}", output)
    // if output.stderr.len() > 0 {
    //     eprintln!("error: {}", String::from_utf8_lossy(&output.stderr).to_string());
    // }
    // String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn run_command(cmd: &str, args: Vec<String>) -> String {
    let output = Command::new(cmd)
                        .args(args)
                        .status()
                        .expect("failed to execute command");
    format!("{:?}", output)
}