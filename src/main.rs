use std::env;
use std::process::Command;
fn main() {
    Command::new("wt")
        .args(&["-p", "PowerShell", include_str!("../conf/pscore.pth")])
        .args(env::args().skip(2))
        .spawn()
        .unwrap();
}
