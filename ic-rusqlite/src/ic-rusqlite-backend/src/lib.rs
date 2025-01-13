use std::cell::RefCell;

use candid::CandidType;
use candid::Deserialize;
use rusqlite::types::Type;
use rusqlite::Connection;
use rusqlite::ToSql;

use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};

thread_local! {
    static DB: RefCell<Option<Connection>> = RefCell::new(None);
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

const MOUNTED_MEMORY_ID: u8 = 20;
const DB_FILE_NAME: &str = "db.db3";

#[ic_cdk::update]
fn add(name: String, data: String, age: u32) {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let db = db.as_mut().unwrap();
        db.execute(
            "INSERT INTO person (name, data, age) VALUES (?1, ?2, ?3)",
            (&name, &data, age),
        )
        .unwrap();
    });
}

#[ic_cdk::query]
fn list() -> Vec<(u64, String, String, u32)> {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let db = db.as_mut().unwrap();
        let mut stmt = db
            .prepare("SELECT id, name, data, age FROM person")
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                ))
            })
            .unwrap();
        let mut result = vec![];
        for person in rows {
            result.push(person.unwrap());
        }
        result
    })
}

#[ic_cdk::query]
fn query(sql: String) -> QueryResult {
    DB.with(|db| {
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
                                Type::Null => vec.push(String::from("")),
                                Type::Integer => vec.push(v.as_i64().unwrap().to_string()),
                                Type::Real => vec.push(v.as_f64().unwrap().to_string()),
                                Type::Text => vec.push(v.as_str().unwrap().parse().unwrap()),
                                Type::Blob => vec.push(hex::encode(v.as_blob().unwrap())),
                            }
                        }
                        res.push(vec)
                    }
                    None => break,
                },
                Err(err) => {
                    return Err(Error::CanisterError {
                        message: format!("{:?}", err),
                    })
                }
            }
        }
        Ok(res)
    })
}

fn mount_memory_files() {
    MEMORY_MANAGER.with(|m| {
        let m = m.borrow();
        ic_wasi_polyfill::init_with_memory_manager(&[0u8; 32], &[], &m, 200..210);

        // mount virtual memory as file for faster DB operations
        let memory = m.get(MemoryId::new(MOUNTED_MEMORY_ID));
        ic_wasi_polyfill::mount_memory_file(DB_FILE_NAME, Box::new(memory));
    });
}

fn open_database() {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        *db = Some(Connection::open(DB_FILE_NAME).unwrap());
    });
}

fn close_database() {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        *db = None;
    });
}

fn create_tables() {
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let db = db.as_mut().unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                data  TEXT,
                age   INTEGER
            )",
            (), // empty list of parameters.
        )
        .unwrap();
    });
}

fn set_pragmas() {
    // set pragmas
    DB.with(|db| {
        let mut db = db.borrow_mut();
        let db = db.as_mut().unwrap();

        // do not create and destroy the journal file every time, set its size to 0 instead
        db.pragma_update(None, "journal_mode", &"TRUNCATE" as &dyn ToSql)
            .unwrap();

        // reduce synchronizations
        db.pragma_update(None, "synchronous", &0 as &dyn ToSql)
            .unwrap();

        // use fewer writes to disk with larger memory chunks
        // Note: values above 16K cause I/O errors for some reason
        db.pragma_update(None, "page_size", &16384 as &dyn ToSql)
            .unwrap();

        // reduce locks and unlocks
        db.pragma_update(None, "locking_mode", &"EXCLUSIVE" as &dyn ToSql)
            .unwrap();

        // temp_store = MEMORY, disables creating temp files, improves performance,
        // this workaround also avoids sqlite error on creating a tmp file on complex queries
        db.pragma_update(None, "temp_store", &2 as &dyn ToSql)
            .unwrap();

        // add this option to minimize disk reads and work in canister memory instead
        //db.pragma_update(None, "cache_size", &1000000 as &dyn ToSql)
        //    .unwrap();
    });
}

#[ic_cdk::init]
fn init() {
    mount_memory_files();

    open_database();
    set_pragmas();
    create_tables();
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    close_database();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    mount_memory_files();

    open_database();
    set_pragmas();
}

#[derive(CandidType, Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

type QueryResult<T = Vec<Vec<String>>, E = Error> = std::result::Result<T, E>;
