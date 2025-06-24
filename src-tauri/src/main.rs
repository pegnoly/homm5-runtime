// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use homm5_runtime_lib::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    homm5_runtime_lib::run().await
}
