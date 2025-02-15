use crate::exec;

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    let s = exec("ps", ["-p", &pid.to_string(), "-o", "ppid=,comm="])?;
    let mut words = s.split_whitespace();
    let ppid = words.next()?;
    let ppid = ppid.trim().parse().ok()?;
    let path = words.next()?;
    let path = path.trim().to_string();
    Some((ppid, path))
}
