use crate::exec;

fn get_id(pid: u32) -> Option<u32> {
    let cmd = format!(
        r#"$p = (Get-CimInstance -ClassName Win32_Process -Filter "ProcessId = {pid}"); Write-Output $p.ParentProcessId"#
    );
    let s = exec("powershell", ["-c", &cmd])?;
    s.trim().parse().ok()
}

fn get_name(pid: u32) -> Option<String> {
    let cmd = format!(
        r#"$p = (Get-CimInstance -ClassName Win32_Process -Filter "ProcessId = {pid}"); Write-Output $p.Name"#
    );
    let s = exec("powershell", ["-c", &cmd])?;
    Some(s.trim().to_string())
}

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    let ppid = get_id(pid)?;
    let name = get_name(ppid)?;
    Some((ppid, name))
}
