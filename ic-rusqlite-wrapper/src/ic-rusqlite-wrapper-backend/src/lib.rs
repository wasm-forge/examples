#[ic_cdk::update]
fn add(name: String, data: String, age: u32) {
    // execute statement with parameters
    ic_rusqlite::execute(
        "INSERT INTO person (name, data, age) VALUES (?1, ?2, ?3)",
        (&name, &data, age),
    )
    .unwrap();
}

#[ic_cdk::query]
fn list() -> Vec<(u64, String, String, u32)> {
    // query for some data and gather into a vector
    ic_rusqlite::query(
        "SELECT id, name, data, age FROM person".to_string(),
        [],
        |row| {
            Ok((
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
            ))
        },
    )
    .unwrap()
}

#[derive(candid::CandidType, candid::Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

type QueryResult<T = Vec<Vec<String>>, E = Error> = std::result::Result<T, E>;

#[ic_cdk::query]
fn query(sql: String) -> QueryResult {
    // we can still access the Rusqlite connection directly, if we want...
    ic_rusqlite::DB.with(|db| {
        let mut db = db.borrow_mut();
        let db = db.as_mut().unwrap();

        let mut stmt = db.prepare(&sql).unwrap();
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
    // database creation
    ic_rusqlite::execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  TEXT,
            age   INTEGER
         )",
        (),
    )
    .unwrap();
}

#[ic_cdk::init]
fn init() {
    // Connect the database and create table
    ic_rusqlite::DbConnectionBuilder::new().connect().unwrap();
    create_tables();
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    // Disconnect the database before upgrade
    ic_rusqlite::disconnect().unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    // Connect to the database with the same settings as before
    ic_rusqlite::DbConnectionBuilder::new().connect().unwrap();
}
