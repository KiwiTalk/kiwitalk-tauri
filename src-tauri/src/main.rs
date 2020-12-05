#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use client::Client;
use config::Config;
use tauri::execute_promise;

mod client;
mod cmd;
mod config;
mod entity;

fn main() {
  let config = load_config("config.json");
  let client = Arc::new(Mutex::new(Client::new(config)));

  tauri::AppBuilder::new()
    .invoke_handler(move |webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => match command {
          Login {
            email,
            password,
            forced,
            callback,
            error,
          } => {
            let client = client.clone();
            execute_promise(
              webview,
              move || {
                let mut client = client.lock().unwrap();
                let status = client.login(&email, &password, forced)?;
                Ok(status.code())
              },
              callback,
              error,
            );
            Ok(())
          }
        },
      }
    })
    .build()
    .run();
}

fn load_config(path: impl AsRef<std::path::Path>) -> Config {
  std::fs::File::open(path)
    .map(serde_json::from_reader)
    .map_or_else(|_| Default::default(), Result::unwrap)
}
