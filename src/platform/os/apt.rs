use crate::platform;
use std::sync::mpsc::Sender;

pub fn install(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    let _ = log.send("updating apt...".into());
    platform::run_elevated("apt-get", &["update"])?;
    let _ = log.send("installing...".into());
    let args: Vec<&str> = [&["install", "-y"], deps].concat();
    platform::run_elevated("apt-get", &args)
}

pub fn uninstall(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    let _ = log.send("removing...".into());
    let args: Vec<&str> = [&["remove", "-y"], deps].concat();
    platform::run_elevated("apt-get", &args)
}
