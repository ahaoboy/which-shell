use crate::exec;

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    let s = exec("ps", ["-p", &pid.to_string(), "-o", "comm,ppid"])?;
    let mut lines = s.lines();
    let ppid = lines.next()?;
    let ppid = ppid.trim().parse().ok()?;
    let path = lines.next()?;
    let path = path.trim().to_string();
    Some((ppid, path))
}
