use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct DbStore {
    conn: Connection,
}

impl DbStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // 初始化表结构
        conn.execute(
            "CREATE TABLE IF NOT EXISTS interactions (
                id INTEGER PRIMARY KEY,
                user_id TEXT NOT NULL,
                session_id TEXT,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS facts (
                id INTEGER PRIMARY KEY,
                user_id TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn save_interaction(&self, user_id: &str, session_id: Option<&str>, content: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO interactions (user_id, session_id, content) VALUES (?1, ?2, ?3)",
            params![user_id, session_id, content],
        )?;
        Ok(())
    }

    pub fn save_fact(&self, user_id: &str, content: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO facts (user_id, content) VALUES (?1, ?2)",
            params![user_id, content],
        )?;
        Ok(())
    }
}
