pub mod models;
pub mod solver;

#[tauri::command]
async fn solve_linear_program(input: models::LinearProgramInput) -> Result<models::SolveResponse, String> {
    // Le solveur utilise num-rational pour une précision absolue
    solver::solve(&input).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![solve_linear_program])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}