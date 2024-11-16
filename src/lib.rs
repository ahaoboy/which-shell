use regex::Regex;
use std::fmt::Display;
use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};
use sysinfo::{Pid, System};

fn exec<I, S>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(cmd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_file_name(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let name = path.split('/').last()?.split('.').next()?.trim();
    Some(name.into())
}

#[derive(Debug, Clone, Copy)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Pwsh,
    Cmd,
    Nu,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ShellVersion {
    pub shell: Shell,
    pub version: Option<String>,
}

impl Display for ShellVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref v) = self.version {
            f.write_str(&format!("{} {}", self.shell, v))
        } else {
            f.write_str(&format!("{}", self.shell))
        }
    }
}

impl From<&str> for Shell {
    fn from(val: &str) -> Self {
        match val {
            "fish" => Shell::Fish,
            "zsh" => Shell::Zsh,
            "OpenConsole" => Shell::PowerShell,
            "powershell" => Shell::PowerShell,
            "bash" => Shell::Bash,
            "pwsh" => Shell::Pwsh,
            "cmd" => Shell::Cmd,
            "nu" => Shell::Nu,
            _ => Shell::Unknown,
        }
    }
}

impl Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Shell::Fish => "fish",
            Shell::Zsh => "zsh",
            Shell::Bash => "bash",
            Shell::PowerShell => "powershell",
            Shell::Cmd => "cmd",
            Shell::Pwsh => "pwsh",
            Shell::Nu => "nu",
            Shell::Unknown => "unknown",
        };
        f.write_str(s)
    }
}

fn get_shell_version(sh: Shell) -> Option<String> {
    let args = match sh {
        Shell::PowerShell => vec!["-c", "$PSVersionTable.PSVersion -replace '\\D', '.'"],
        _ => vec!["--version"],
    };
    let version = exec(sh.to_string().as_str(), args)?;
    match sh {
        Shell::Fish => {
            // fish, version 3.6.1
            return Some(version[14..].trim().into());
        }
        Shell::Pwsh => {
            // PowerShell 7.4.1
            return Some(version[11..].trim().into());
        }
        Shell::Bash => {
            // GNU bash, version 5.2.26(1)-release (aarch64-unknown-linux-android)
            let re = Regex::new(r"([0-9]+).([0-9]+).([0-9]+)").unwrap();
            let cap = re.captures(&version)?;

            if let (Some(a), Some(b), Some(c)) = (cap.get(1), cap.get(2), cap.get(3)) {
                return Some(format!("{}.{}.{}", a.as_str(), b.as_str(), c.as_str()));
            }
            None
        }
        Shell::Cmd => {
            // Microsoft Windows [版本 10.0.22635.2700]
            // (c) Microsoft Corporation。保留所有权利。
            let s = version
                .lines()
                .next()?
                .split(' ')
                .last()?
                .split(']')
                .next()?
                .trim();
            Some(s.into())
        }
        // 5.1.26100.2161
        Shell::PowerShell => Some(version),
        Shell::Nu => {
            // 0.99.0
            Some(version)
        }
        _ => None,
    }
}

pub fn which_shell() -> Option<ShellVersion> {
    let system = System::new_all();
    // system.refresh_all();
    let mut pid = std::process::id() as usize;
    while let Some(process) = system.process(Pid::from(pid)) {
        let path = process.exe()?.to_str()?;
        let cmd = get_file_name(path)?;
        let shell: Shell = cmd.as_str().into();
        match shell {
            Shell::Unknown => {
                if let Some(parent_id) = process.parent() {
                    pid = parent_id.as_u32() as usize;
                } else {
                    break;
                }
                continue;
            }
            _ => {
                let version = get_shell_version(shell);
                return Some(ShellVersion { shell, version });
            }
        }
    }
    None
}
