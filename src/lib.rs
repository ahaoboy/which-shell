use regex::Regex;
use std::fmt::Display;
use std::{ffi::OsStr, process::Command};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
use unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::*;

fn exec<I, S>(cmd: S, args: I) -> Option<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(cmd)
        .args(args)
        .envs(std::env::vars())
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Some(s)
}

fn get_file_name(path: &str) -> Option<String> {
    let path = path.replace('\\', "/");
    let name = path.split('/').last()?.split('.').next()?.trim();
    Some(name.into())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Pwsh,
    Cmd,
    Nu,
    Dash,
    Ksh,
    Tcsh,
    Csh,
    Sh,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            "dash" => Shell::Dash,
            "ksh" => Shell::Ksh,
            "ksh93" => Shell::Ksh,
            "tcsh" => Shell::Tcsh,
            "csh" => Shell::Csh,
            "bsd-csh" => Shell::Csh,
            "sh" => Shell::Sh,
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
            Shell::Dash => "dash",
            Shell::Ksh => "ksh",
            Shell::Tcsh => "tcsh",
            Shell::Csh => "csh",
            Shell::Sh => "sh",
            Shell::Unknown => "unknown",
        };
        f.write_str(s)
    }
}

fn get_shell_version(sh: Shell) -> Option<String> {
    let args = match sh {
        Shell::PowerShell => vec!["-c", "$PSVersionTable.PSVersion -replace '\\D', '.'"],
        Shell::Ksh => vec!["-c", "echo $KSH_VERSION"],
        _ => vec!["--version"],
    };
    let version = exec(sh.to_string().as_str(), args)?;
    match sh {
        Shell::Fish => {
            // fish, version 3.6.1
            Some(version[14..].trim().into())
        }
        Shell::Pwsh => {
            // PowerShell 7.4.1
            Some(version[11..].trim().into())
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
        Shell::PowerShell => {
            // 5.1.26100.2161
            Some(version)
        }
        Shell::Nu => {
            // 0.99.0
            Some(version)
        }
        Shell::Ksh => {
            // Version AJM 93u+m/1.0.8 2024-01-01
            let v = version.split("/").nth(1)?;
            let v = v.split(" ").next().map(|s| s.trim().to_string());
            v
        }
        Shell::Zsh => {
            // zsh 5.9 (x86_64-ubuntu-linux-gnu)
            let v = version.split(" ").nth(1).map(|s| s.trim().to_string());
            v
        }
        Shell::Tcsh => {
            // tcsh 6.24.13 (Astron) 2024-06-12 (x86_64-unknown-linux) options wide,nls,dl,al,kan,sm,rh,nd,color,filec
            let v = version.split(" ").nth(1).map(|s| s.trim().to_string());
            v
        }
        _ => None,
    }
}

pub fn which_shell() -> Option<ShellVersion> {
    let mut pid = std::process::id();
    while let Some((ppid, path)) = get_ppid(pid) {
        let cmd = get_file_name(&path)?;
        let shell: Shell = cmd.as_str().into();
        match shell {
            Shell::Unknown => {
                pid = ppid;
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
