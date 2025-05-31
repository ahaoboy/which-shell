use std::fs::read_to_string;

fn get_id(pid: u32) -> Option<u32> {
    read_to_string(format!("/proc/{pid}/status"))
        .ok()
        .and_then(|s| {
            for line in s.lines() {
                let header = "PPid:";
                if line.starts_with("PPid:") {
                    if let Ok(n) = line[header.len()..].trim().parse() {
                        return Some(n);
                    }
                }
            }
            None
        })
}

fn get_name(pid: u32) -> Option<String> {
    read_to_string(format!("/proc/{pid}/comm")).ok()
}

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    let ppid = get_id(pid)?;
    let name = get_name(ppid)?;
    Some((ppid, name))
}
