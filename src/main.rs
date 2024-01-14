#![windows_subsystem = "windows"]
use axum::{
    extract::Query,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct Bullshit {
    url: String,
    title: String,
}
#[derive(Deserialize, Debug, Clone)]
struct Directories {
    ytdlp: String,
    download: String,
}

#[tokio::main]
async fn main() {
    let data = fs::read_to_string("./directories.json").expect("Unable to read file");
    let directories: Directories = serde_json::from_str(&data).expect("JSON is fucked");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/fuml", get(move |q| list_things(directories, q)))
        .layer(CorsLayer::permissive());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// This will parse query strings like `?page=2&per_page=30` into `Pagination`
// structs.
async fn list_things(directories: Directories, fuml: Query<Bullshit>) {
    let Bullshit { url, title } = fuml.0;

    // Execute yt-dlp with the specified video url, downloading the video to the specified title as a filename
    let output = Command::new(directories.ytdlp)
        .arg(&url)
        .arg("-o")
        .arg(&format!("{}\\{}.%(ext)s", directories.download, title))
        .output()
        .unwrap();

    // Check if the command was successful
    // Check if the command was successful
    println!("stdout: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {:?}", String::from_utf8(output.stderr).unwrap());

    if output.status.success() {
        println!("yt-dlp command executed successfully!");
    } else {
        eprintln!("Error: yt-dlp command failed with {:?}", output.status);
    }
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello you are currently running cum.exe! (Version 69.4.20)"
}
