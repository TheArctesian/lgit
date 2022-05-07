use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Message {
    message: String,
}
fn main() {
    let args = Message::parse();
    let git_add = Command::new("git").arg("add").arg(".").output().unwrap();
    let git_commit = Command::new("git").arg("commit").arg("-m").arg(args.message).output().unwrap();
    let git_push = Command::new("git").arg("push").output().unwrap();
    println!("{:?}", git_push);
}
