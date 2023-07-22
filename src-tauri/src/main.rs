// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn youtube_dl(url: &str, destination: &str) -> Result<String, String> {
    let status = Command::new("/opt/homebrew/bin/yt-dlp")
        .args(["-f", "bestaudio/best"])
        .arg("-ciw")
        .args(["-o", "%(title)s.%(ext)s"])
        .arg("-v")
        .arg("--extract-audio")
        .args(["--audio-quality", "0"])
        .args(["--audio-format", "mp3"])
        .args(["-P", destination])
        .arg(url)
        .status()
        .map_err(|err| err.to_string())?;

    Ok(status.to_string())
}

#[tauri::command]
fn soundcloud_dl(url: &str, destination: &str) -> Result<String, String> {
    println!("url: {}", url);
    println!("destination: {}", destination);
    let status = Command::new("/opt/homebrew/bin/scdl")
        .args(["--path", destination])
        .args(["-l", url])
        .arg("-c")
        .status()
        .map_err(|err| err.to_string())?;

    Ok(status.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![youtube_dl, soundcloud_dl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
