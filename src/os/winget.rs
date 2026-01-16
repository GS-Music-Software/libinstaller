use std::process::Command;
use std::sync::mpsc::Sender;

const PACKAGES: &[(&str, &str)] = &[
    ("mpv", "mpv.net"),
    ("yt-dlp", "yt-dlp.yt-dlp"),
    ("ffmpeg", "Gyan.FFmpeg"),
];

fn get_package_id(dep: &str) -> Option<&'static str> {
    PACKAGES.iter().find(|(name, _)| *name == dep).map(|(_, id)| *id)
}

pub fn install(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    for dep in deps {
        let Some(pkg) = get_package_id(dep) else { continue };
        let _ = log.send(format!("installing {}...", dep));
        let status = Command::new("winget")
            .args(["install", "--id", pkg, "-e", "--silent", "--accept-package-agreements", "--accept-source-agreements"])
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("failed to install {}", dep));
        }
    }
    Ok(())
}

pub fn uninstall(deps: &[&str], log: &Sender<String>) -> Result<(), String> {
    for dep in deps {
        let Some(pkg) = get_package_id(dep) else { continue };
        let _ = log.send(format!("removing {}...", dep));
        let status = Command::new("winget")
            .args(["uninstall", "--id", pkg, "-e", "--silent"])
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("failed to remove {}", dep));
        }
    }
    Ok(())
}
