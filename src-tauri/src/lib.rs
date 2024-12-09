mod entities;

use std::fmt::format;

use serde_json::json;

use syntax_analyzer_core::core::analyzer;
use syntax_analyzer_core::cli::error;

use crate::entities::{Message, AnalyzeError, ErrorType, AnalyzeSuccess};

#[tauri::command]
fn analyze(chain: &str) -> Result<String, String> {
    match analyzer::analyze(chain, ';') {
        Err((index, message)) => {
            let html = error::with_message_html(chain, index, message);

            let result = AnalyzeError {
                error_type: ErrorType::Syntax,
                index: index as i32,
                message: Message {
                    plain: message.to_string(),
                    html,
                },
            };

            let json = json!(&result).to_string();
            Err(json)
        },
        Ok(_) => {
            let plain = format!("\"{}\" is chain", chain);
            let html = format!("<span class='right'>\"{}\" is chain</span>", chain);

            let result = AnalyzeSuccess {
                semantics: String::from("foo bar"),
                message: Message {
                    plain,
                    html,
                }
            };

            let json = json!(&result).to_string();
            Ok(json)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![analyze])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
