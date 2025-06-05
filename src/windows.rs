use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    },
};

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    unsafe {
        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .ok()
            .unwrap_or(INVALID_HANDLE_VALUE);
        if snapshot == INVALID_HANDLE_VALUE {
            return None;
        }

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_ok() {
            loop {
                if entry.th32ProcessID == pid {
                    let name = {
                        let len = (0..entry.szExeFile.len())
                            .position(|i| entry.szExeFile[i] == 0)
                            .unwrap_or(entry.szExeFile.len());
                        OsString::from_wide(&entry.szExeFile[..len])
                            .to_string_lossy()
                            .into_owned()
                    };
                    let ppid = entry.th32ParentProcessID;
                    CloseHandle(snapshot).ok()?;
                    return Some((ppid, name));
                }
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }

        CloseHandle(snapshot).ok()?;
        None
    }
}
