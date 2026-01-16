use crate::platform;
use std::sync::mpsc::Sender;

pub fn install(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    let _ = log.send("using dnf...".into());
    let args: Vec<&str> = [&["install", "-y"], deps].concat();
    platform::run_elevated("dnf", &args)
}

pub fn uninstall(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    let _ = log.send("using dnf...".into());
    let args: Vec<&str> = [&["remove", "-y"], deps].concat();
    platform::run_elevated("dnf", &args)
}
