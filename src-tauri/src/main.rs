// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn youtube_dl(url: &str, destination: &str) -> Result<String, String> {
    let child = Command::new("/opt/homebrew/bin/yt-dlp")
        .args(["-f", "bestaudio/best"])
        .arg("-ciw")
        .args(["-o", "%(title)s.%(ext)s"])
        .arg("-v")
        .arg("--extract-audio")
        .args(["--audio-quality", "0"])
        .args(["--audio-format", "mp3"])
        .args(["-P", destination])
        .arg(url)
        .spawn()
        .map_err(|err| err.to_string())?;

    Ok(child.id().to_string())
}

#[tauri::command]
fn soundcloud_dl(url: &str, destination: &str) -> Result<String, String> {
    println!("url: {}", url);
    println!("destination: {}", destination);
    let child = Command::new("/opt/homebrew/bin/scdl")
        .args(["--path", destination])
        .args(["-l", url])
        .arg("-c")
        .spawn()
        .map_err(|err| err.to_string())?;

    Ok(child.id().to_string())
}

#[tauri::command]
fn kill_process(pid: &str) -> Result<String, String> {
    println!("pid: {}", pid);
    let child = Command::new("kill")
        .arg("-9")
        .arg(pid)
        .spawn()
        .map_err(|err| err.to_string())?;

    Ok(child.id().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![youtube_dl, soundcloud_dl, kill_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
