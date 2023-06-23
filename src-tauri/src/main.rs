// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn youtube_dl(url: &str, destination: &str) -> String {
    let status = Command::new("yt-dlp")
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
        .expect("failed to execute process");


    return status.to_string();
}

#[tauri::command]
fn soundcloud_dl(url: &str, destination: &str) -> String {
    println!("url: {}", url);
    println!("destination: {}", destination);
    let status = Command::new("scdl")
        .args(["--path", destination])
        .args(["-l", url])
        .arg("-c")
        .status()
        .expect("failed to execute process");

    return status.to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![youtube_dl, soundcloud_dl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
