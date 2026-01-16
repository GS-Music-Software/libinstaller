#[derive(Clone, Copy, PartialEq)]
pub enum PackageManager {
    Dnf,
    Apt,
    Pacman,
    Zypper,
    Winget,
    Choco,
    Unknown,
}

pub fn detect_package_manager() -> PackageManager {
    if cfg!(target_os = "windows") {
        windows()
    } else {
        linux()
    }
}

fn linux() -> PackageManager {
    if which::which("dnf").is_ok() {
        PackageManager::Dnf
    } else if which::which("apt-get").is_ok() {
        PackageManager::Apt
    } else if which::which("pacman").is_ok() {
        PackageManager::Pacman
    } else if which::which("zypper").is_ok() {
        PackageManager::Zypper
    } else {
        PackageManager::Unknown
    }
}

fn windows() -> PackageManager {
    if which::which("winget").is_ok() {
        PackageManager::Winget
    } else if which::which("choco").is_ok() {
        PackageManager::Choco
    } else {
        PackageManager::Unknown
    }
}

pub fn is_installed(name: &str) -> bool {
    let cmd = if name == "ffmpeg" { "ffprobe" } else { name };
    which::which(cmd).is_ok()
}
