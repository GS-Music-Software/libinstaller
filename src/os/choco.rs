use crate::platform;
use std::sync::mpsc::Sender;

pub fn install(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    for dep in deps {
        let _ = log.send(format!("installing {}...", dep));
        platform::run_elevated("choco", &["install", dep, "-y"])?;
    }
    Ok(())
}

pub fn uninstall(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    for dep in deps {
        let _ = log.send(format!("removing {}...", dep));
        platform::run_elevated("choco", &["uninstall", dep, "-y"])?;
    }
    Ok(())
}
