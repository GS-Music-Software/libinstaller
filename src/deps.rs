use crate::platform;

const LINUX: &[&str] = &["mpv", "yt-dlp", "ffmpeg", "socat"];
const WINDOWS: &[&str] = &["mpv", "yt-dlp", "ffmpeg"];

fn list() -> &'static [&'static str] {
    if cfg!(target_os = "windows") {
        WINDOWS
    } else {
        LINUX
    }
}

pub fn missing() -> Vec<&'static str> {
    list()
        .iter()
        .filter(|&&dep| !platform::is_installed(dep))
        .copied()
        .collect()
}

pub fn installed() -> Vec<&'static str> {
    list()
        .iter()
        .filter(|&&dep| platform::is_installed(dep))
        .copied()
        .collect()
}

pub fn all_installed() -> bool {
    list().iter().all(|&dep| platform::is_installed(dep))
}
