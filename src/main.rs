use std::{
    env::var,
    io::{stderr, stdout, Write},
    process::{exit, Command, Output},
};

const DEFAULT_BRANCH: &str = "default";
const DEFAULT_REMOTE: &str = "origin";

fn main() {
    git_version();
    git_init();
    git_remote();
    git_fetch();
    git_checkout();
    git_log();
}

fn git_version() {
    let command = "git --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
}

fn git_init() {
    let command = format!("git init --initial-branch={}", DEFAULT_BRANCH);
    exec(command);
}

fn git_remote() {
    let server = env("GITHUB_SERVER_URL");
    let repository = env("GITHUB_REPOSITORY");
    let command = format!(
        "git remote add {} {}/{}",
        DEFAULT_REMOTE, server, repository
    );
    exec(command);
}

fn git_fetch() {
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
}

fn git_checkout() {
    let command = "git checkout";
    exec(command);
}

fn git_log() {
    let command = "git --no-pager log --date=iso --no-decorate";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
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
        eprintln!("--- status ---");
        eprintln!("{}", output.status);
        eprintln!();
        eprintln!("--- stdout ---");
        stderr().write_all(&output.stdout).unwrap_or_else(|error| {
            eprintln!("stderr error: {}", error);
            exit(1);
        });
        eprintln!();
        eprintln!("--- stderr ---");
        stderr().write_all(&output.stderr).unwrap_or_else(|error| {
            eprintln!("stderr error: {}", error);
            exit(1);
        });
        eprintln!();
        exit(1);
    }
}

fn env(name: &str) -> String {
    var(name).unwrap_or_else(|error| {
        eprintln!("environment variable error: {}", error);
        exit(1);
    })
}
