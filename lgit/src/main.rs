use clap::Parser;
use std::process::Command;

#[derive(Parser)]
struct Message {
    message: String,
}
fn main() {
    let args = Message::parse();
    let gitAdd = Command::new("git").arg("add").arg(".").output().unwrap();
    let gitCommit = Command::new("git").arg("commit").arg("-m").arg(args.message).output().unwrap();
    let gitPush = Command::new("git").arg("push").output().unwrap();
    println!("{:?}", gitAdd);
    println!("{:?}", gitCommit);
    println!("{:?}", gitPush);

}
