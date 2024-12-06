use syntax_analyzer_core::core::analyzer::analyze;
use syntax_analyzer_core::cli::error::with_message_html;

#[tauri::command]
fn check(chain: &str) -> Result<String, String> {
    match analyze(chain, ';') {
        Err((index, message)) => {
            let result = with_message_html(chain, index, message);
            Err(result)
        },
        Ok(_) => {
            let result = format!("<span class='right'>\"{}\" is chain</span>", chain);
            Ok(result)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
