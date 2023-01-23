#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{error::Error, collections::HashMap};

use tiger_analyzer::AppBuilder;
use tauri::{LogicalSize, Size, Manager, Window};

fn main() {
    AppBuilder::new().run();      
}
