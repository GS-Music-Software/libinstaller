use crate::deps;
use crate::os;
use crate::platform::PackageManager;
use std::sync::mpsc::Sender;

pub fn run(pm: PackageManager, log: Sender<String>) -> Result<(), String> {
    let missing = deps::missing();

    if missing.is_empty() {
        let _ = log.send("all dependencies installed".into());
        return Ok(());
    }

    let _ = log.send(format!("installing: {}", missing.join(", ")));

    match pm {
        PackageManager::Dnf => os::dnf::install(&missing, &log),
        PackageManager::Apt => os::apt::install(&missing, &log),
        PackageManager::Pacman => os::pacman::install(&missing, &log),
        PackageManager::Zypper => os::zypper::install(&missing, &log),
        PackageManager::Winget => os::winget::install(&missing, &log),
        PackageManager::Choco => os::choco::install(&missing, &log),
        PackageManager::Unknown => Err("unsupported".into()),
    }
}
