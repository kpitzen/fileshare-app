use serde::{Deserialize, Serialize};
use sqlx::{types, FromRow, PgPool};

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct File {
    pub id: i32,
    pub filename: String,
    pub created_at: types::chrono::DateTime<types::chrono::Utc>,
    pub file_object: Option<Vec<u8>>,
}

impl File {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<File>, sqlx::Error> {
        let mut files: Vec<File> = vec![];
        let recs = sqlx::query_as(r#"SELECT id, filename, created_at, file_object FROM files;"#)
            .fetch_all(pool)
            .await?;
        for rec in recs {
            files.push(rec);
        }

        Ok(files)
    }
}
