use std::env;
use std::process::Command;
fn main() {
    let mut rest_args: Vec<String> = env::args().skip(2).collect();
    if rest_args
        .first()
        .unwrap_or(&String::from("-"))
        .chars()
        .nth(0)
        .unwrap()
        != '-'
    {
        rest_args.insert(0, String::from("-Command"));
    }
    Command::new("wt")
        .args(&["-p", "PowerShell", include_str!("../conf/pscore.pth")])
        .args(rest_args)
        .spawn()
        .unwrap();
}
