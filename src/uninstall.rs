use crate::deps;
use crate::os;
use crate::platform::PackageManager;
use std::sync::mpsc::Sender;

pub fn run(pm: PackageManager, log: Sender<String>) -> Result<(), String> {
    let installed = deps::installed();

    if installed.is_empty() {
        let _ = log.send("nothing to remove".into());
        return Ok(());
    }

    let _ = log.send(format!("removing: {}", installed.join(", ")));

    match pm {
        PackageManager::Dnf => os::dnf::uninstall(&installed, &log),
        PackageManager::Apt => os::apt::uninstall(&installed, &log),
        PackageManager::Pacman => os::pacman::uninstall(&installed, &log),
        PackageManager::Zypper => os::zypper::uninstall(&installed, &log),
        PackageManager::Winget => os::winget::uninstall(&installed, &log),
        PackageManager::Choco => os::choco::uninstall(&installed, &log),
        PackageManager::Unknown => Err("unsupported".into()),
    }
}
