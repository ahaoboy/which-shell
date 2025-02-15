use crate::exec;

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    let cmd = format!(
        r#"$p = (Get-CimInstance -ClassName Win32_Process -Filter "ProcessId = {pid}"); Write-Output $p.ParentProcessId $p.Name"#
    );
    let s = exec("powershell", ["-c", &cmd])?;
    let mut lines = s.lines();
    let ppid = lines.next()?;
    let ppid = ppid.trim().parse().ok()?;
    let path = lines.next()?;
    let path = path.trim().to_string();
    Some((ppid, path))
}
