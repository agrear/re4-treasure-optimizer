// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;

use app::*;

#[tauri::command]
fn get_gems() -> Vec<&'static Gem> {
    GEMS.iter().collect()
}

#[tauri::command]
fn get_treasures() -> Vec<&'static Treasure> {
    TREASURES.iter().collect()
}

#[tauri::command]
fn optimize(
    treasures: TreasureCollection,
    gems: Combo,
    objective_function: ObjectiveFunction
) -> Vec<SocketedTreasure> {
    let result = allocate_gems(
        treasures,
        gems,
        objective_function
    );

    if result.is_none() {
        panic!("Something went wrong");
    }

    result.unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_gems,
            get_treasures,
            optimize
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
