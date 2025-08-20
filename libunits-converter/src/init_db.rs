// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) async fn init_from_scratch(db_path: &str, migrations_path: &str) -> turso::Result<()> {
    let filename = format!("{}/sqlite.db", db_path);
    if !std::fs::exists(&filename).unwrap() {
        println!("Prepare databse");
        let db = turso::Builder::new_local(&filename).build().await?;

        let conn = db.connect()?;
        let migrations_list = [
            format!("{}/002_dimensions.sql", migrations_path),
            format!("{}/003_conversion.sql", migrations_path),
        ];
        for migration in migrations_list {
            let sql_content = std::fs::read_to_string(migration).unwrap();
            let queries: Vec<&str> = sql_content
                .split(';')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            for query in queries {
                conn.execute(query, ()).await?;
                println!("Executed : {}", query);
            }
        }
    } else {
        eprintln!("Database already set");
    }
    let mut perms = std::fs::metadata(&filename).unwrap().permissions();
    perms.set_readonly(true);
    // std::fs::set_permissions(filename, perms).unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let db_path = match std::env::var("DB_FILE_PATH") {
        Ok(path) => path,
        Err(std::env::VarError::NotPresent) => {
            let path = format!("{}/data", root);
            eprintln!("Using default database");
            path
        }
        Err(e) => panic!("{}", e),
    };
    std::fs::create_dir_all(&db_path).unwrap();

    let migrations_path = format!("{}/migrations/", root);

    init_from_scratch(&db_path, &migrations_path).await.unwrap();

    println!("Created databse at : {}", db_path);
}
