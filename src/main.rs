use std::{
    env::var,
    io::{stderr, stdout, Write},
    process::{exit, Command, Output},
};

const DEFAULT_BRANCH: &str = "default";
const DEFAULT_REMOTE: &str = "origin";

fn main() {
    // git version
    let command = "git --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap();
    // initialize repository
    let command = format!("git init --initial-branch={}", DEFAULT_BRANCH);
    exec(command);
    // add remote
    let server = env("GITHUB_SERVER_URL");
    let repository = env("GITHUB_REPOSITORY");
    let command = format!(
        "git remote add {} {}/{}",
        DEFAULT_REMOTE, server, repository
    );
    exec(command);
    // fetch origin
    let refspec = env("GITHUB_REF");
    let command = format!(
        "git fetch \
        --depth=1 \
        --no-tags \
        --update-head-ok \
        {} +{}:{}",
        DEFAULT_REMOTE, refspec, DEFAULT_BRANCH
    );
    exec(command);
    // checkout default branch
    let command = "git checkout";
    exec(command);
    // show
    let command = "git --no-pager log --date=iso --no-decorate";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap();
}

fn exec<S: AsRef<str>>(command: S) -> Output {
    let command = command.as_ref();
    println!("❯ {}", command);
    let split: Vec<_> = command.trim().split_whitespace().collect();
    let (executable, arguments) = split.split_first().unwrap();
    let output = Command::new(executable).args(arguments).output().unwrap();
    if output.status.success() {
        output
    } else {
        eprintln!("command failed");
        eprintln!("--- source ---");
        eprintln!("{}", command);
        eprintln!("--- status ---");
        eprintln!("{}", output.status);
        eprintln!();
        eprintln!("--- stdout ---");
        stderr().write_all(&output.stdout).unwrap();
        eprintln!();
        eprintln!("--- stderr ---");
        stderr().write_all(&output.stderr).unwrap();
        eprintln!();
        exit(1);
    }
}

fn env(name: &str) -> String {
    match var(name) {
        Ok(var) => var,
        Err(e) => {
            eprintln!(
                "failed to interpret environment variable \"{}\": {}",
                name, e
            );
            exit(1);
        }
    }
}
