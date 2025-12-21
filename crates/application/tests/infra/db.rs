use std::path::PathBuf;

pub async fn build_test_database(
    test_file: &str,
    test_name: &str,
) -> anyhow::Result<database::SqlitePool> {
    let path = test_db_path(test_file, test_name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    let db_file = match path.to_str() {
        Some(p) => p,
        None => {
            return Err(anyhow::anyhow!("path cannot be converted to str"));
        }
    };
    let pool = database::setup::setup_database(&db_file).await?;
    Ok(pool)
}

fn test_db_path(test_file: &str, test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-db")
        .join(test_file.replace(".rs", ""))
        .join(format!("{test_name}.sqlite3"))
}
