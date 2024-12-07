use serde::{Serialize};

#[derive(Serialize)]
pub enum ErrorType {
    Syntax,
    Semantic,
}

#[derive(Serialize)]
pub struct Message {
    pub plain: String,
    pub html: String
}

#[derive(Serialize)]
pub struct AnalyzeError {
    pub error_type: ErrorType,
    pub index: i32,
    pub message: Message
}

#[derive(Serialize)]
pub struct AnalyzeSuccess {
    pub message: Message
}
