use ic_rusqlite::with_connection;

#[ic_cdk::update]
fn add(name: String, data: String, age: u32) {
    // execute statement with parameters

    ic_rusqlite::with_connection(|conn| {
        conn.execute(
            "INSERT INTO person (name, data, age) VALUES (?1, ?2, ?3)",
            (&name, &data, age),
        )
        .unwrap();
    })
}

#[ic_cdk::query]
fn list() -> Vec<(i64, String, String, u32)> {
    // get connection
    ic_rusqlite::with_connection(|conn| {
        // prepare statement
        let mut stmt = conn
            .prepare("SELECT id, name, data, age FROM person")
            .unwrap();

        //
        let iter = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .unwrap();

        iter.map(|r| r.unwrap()).collect()
    })
}

#[derive(candid::CandidType, candid::Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

type QueryResult<T = Vec<Vec<String>>, E = Error> = std::result::Result<T, E>;

#[ic_cdk::query]
fn query(sql: String) -> QueryResult {
    // get connection
    ic_rusqlite::with_connection(|conn| {
        let mut stmt = conn.prepare(&sql).unwrap();
        let cnt = stmt.column_count();
        let mut rows = stmt.query([]).unwrap();
        let mut res: Vec<Vec<String>> = Vec::new();

        loop {
            match rows.next() {
                Ok(row) => match row {
                    Some(row) => {
                        let mut vec: Vec<String> = Vec::new();
                        for idx in 0..cnt {
                            let v = row.get_ref_unwrap(idx);
                            match v.data_type() {
                                ic_rusqlite::rusqlite::types::Type::Null => {
                                    vec.push(String::from(""))
                                }
                                ic_rusqlite::rusqlite::types::Type::Integer => {
                                    vec.push(v.as_i64().unwrap().to_string())
                                }
                                ic_rusqlite::rusqlite::types::Type::Real => {
                                    vec.push(v.as_f64().unwrap().to_string())
                                }
                                ic_rusqlite::rusqlite::types::Type::Text => {
                                    vec.push(v.as_str().unwrap().parse().unwrap())
                                }
                                ic_rusqlite::rusqlite::types::Type::Blob => {
                                    vec.push(hex::encode(v.as_blob().unwrap()))
                                }
                            }
                        }
                        res.push(vec)
                    }
                    None => break,
                },
                Err(err) => {
                    return Err(Error::CanisterError {
                        message: format!("{err:?}"),
                    })
                }
            }
        }
        Ok(res)
    })
}

fn create_tables() {
    with_connection(|conn| {
        // database creation
        conn.execute(
            "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  TEXT,
            age   INTEGER
         )",
            (),
        )
        .unwrap();
    })
}

#[ic_cdk::init]
fn init() {
    // no need to do anything special by default, just create the initial tables
    create_tables();
}
