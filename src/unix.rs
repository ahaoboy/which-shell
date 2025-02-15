use crate::exec;

fn get_id(pid: u32) -> Option<u32> {
  let s = exec("ps", ["-p", &pid.to_string(), "-o", "ppid="])?;
  s.trim().parse().ok()
}

fn get_name(pid: u32) -> Option<String> {
  let s = exec("ps", ["-p", &pid.to_string(), "-o", "comm="])?;
  Some(s.trim().to_string())
}

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
  let ppid = get_id(pid)?;
  let name = get_name(ppid)?;
  Some((ppid, name))
}
