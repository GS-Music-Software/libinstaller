use std::process::Command;

pub fn run_elevated(cmd: &str, args: &[&str]) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        windows(cmd, args)
    } else {
        linux(cmd, args)
    }
}

fn linux(cmd: &str, args: &[&str]) -> Result<(), String> {
    let sudo = if which::which("pkexec").is_ok() {
        "pkexec"
    } else if which::which("sudo").is_ok() {
        "sudo"
    } else {
        return Err("no sudo or pkexec".into());
    };

    let mut full_args = vec![cmd];
    full_args.extend(args);

    let status = Command::new(sudo)
        .args(&full_args)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("command failed".into())
    }
}

#[cfg(target_os = "windows")]
fn windows(cmd: &str, args: &[&str]) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let args_str = args.join(" ");
    let ps_cmd = format!("Start-Process '{}' -ArgumentList '{}' -Verb RunAs -Wait", cmd, args_str);

    let status = Command::new("powershell")
        .args(["-Command", &ps_cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("command failed".into())
    }
}

#[cfg(not(target_os = "windows"))]
fn windows(_cmd: &str, _args: &[&str]) -> Result<(), String> {
    Err("not windows".into())
}
