use crate::platform;
use std::sync::mpsc::Sender;

pub fn install(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    let _ = log.send("using pacman...".into());
    let args: Vec<&str> = [&["-Sy", "--noconfirm"], deps].concat();
    platform::run_elevated("pacman", &args)
}

pub fn uninstall(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    let _ = log.send("using pacman...".into());
    let args: Vec<&str> = [&["-Rs", "--noconfirm"], deps].concat();
    platform::run_elevated("pacman", &args)
}
