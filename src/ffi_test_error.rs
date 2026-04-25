//use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
enum TestErr {
    #[error("InvalidInput: {0}")]
    InvalidInput(String),

    #[error("DbError: {0}")]
    DbError(String),

    #[error("JsonError: {0}")]
    JsonError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Unexpected error: {0}")]
    Other(String),
}



// アトリビュートで自動生成？
// impl ToString for TestErr {
//     fn to_string(&self) -> String {
//         match self {
//             TestErr::InvalidInput(e) => format!("InvalidInput: {}", e),
//             TestErr::DbError(e) => format!("DBError: {}", e),
//             TestErr::JsonError(e) => format!("JsonError: {}", e),
//         }
//     }
// }

// impl fmt::Display for TestErr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TestErr::InvalidInput(e) => write!(f, "InvalidInput: {}", e),
//             TestErr::DbError(e) => format!(f, "DBError: {}", e),
//             TestErr::JsonError(e) => format!(f, "JsonError: {}", e),
//         }
//     }
// }

// impl From<std::io::Error> for TestErr {
//     fn from(e: std::io::Error) -> self{
//         TestErr::io(e.to_string())
//     }
// }