use dioxus::prelude::*;

#[server]
pub async fn save_dog(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO hot_dog (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: std::sync::Arc<rusqlite::Connection> = {
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");
        conn.execute_batch("CREATE TABLE IF NOT EXISTS hot_dog (id INTEGER PRIMARY KEY, url TEXT NOT NULL);")
            .expect("Failed to create table");
        std::sync::Arc::new(conn)
    };
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM hot_dog ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}
