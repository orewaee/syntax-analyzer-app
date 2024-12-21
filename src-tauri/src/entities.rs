use serde::{Serialize};

#[derive(Serialize)]
pub struct Message {
    pub plain: String,
    pub html: String
}

#[derive(Serialize)]
pub struct AnalyzeError {
    pub index: i32,
    pub message: Message
}

#[derive(Serialize)]
pub struct AnalyzeSuccess {
    pub semantics: String,
    pub message: Message
}
