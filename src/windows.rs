use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "Win32_Process")]
pub struct Proc {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ParentProcessId")]
    pub pid: u32,
}

pub fn get_ppid(pid: u32) -> Option<(u32, String)> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con).unwrap();

    let query = format!("SELECT Name,ParentProcessId FROM Win32_Process WHERE ProcessId = {pid}");
    let results: Vec<Proc> = wmi_con.raw_query(&query).unwrap();

    results
        .first()
        .map(|process| (process.pid, process.name.clone()))
}

#[cfg(feature = "async")]
pub async fn get_ppid_async(pid: u32) -> Option<(u32, String)> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con).unwrap();

    let query = format!("SELECT Name,ParentProcessId FROM Win32_Process WHERE ProcessId = {pid}");
    let results: Vec<Proc> = wmi_con.async_raw_query(&query).await.unwrap();

    results
        .first()
        .map(|process| (process.pid, process.name.clone()))
}
