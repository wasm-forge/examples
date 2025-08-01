use std::cell::RefCell;
use std::rc::Rc;

use candid::CandidType;
use candid::Deserialize;

use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};

use turso::Connection;

thread_local! {
    static CONNECTION: RefCell<Option<Rc<Connection>>> = const { RefCell::new(None) };

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

const MOUNTED_MEMORY_ID: u8 = 20;
const DB_FILE_NAME: &str = "db.db3";

pub async fn init_db() -> Rc<Connection> {
    let db = turso::Builder::new_local(DB_FILE_NAME)
        .build()
        .await
        .unwrap();

    let connection = Rc::new(db.connect().unwrap());

    CONNECTION.with_borrow_mut(|c| {
        *c = Some(Rc::clone(&connection));
    });

    connection
}

pub async fn get_connection() -> Rc<Connection> {
    if let Some(conn) = CONNECTION.with_borrow(|c| c.clone()) {
        Rc::clone(&conn)
    } else {
        init_db().await
    }
}

fn close_database() {
    CONNECTION.with_borrow_mut(|db| {
        *db = None;
    });
}

#[ic_cdk::update]
async fn add(name: String, data: String, age: u32) {
    let conn = get_connection().await;

    conn.execute(
        "INSERT INTO person (name, data, age) VALUES (?1, ?2, ?3);",
        (name, data, age),
    )
    .await
    .unwrap();
}

#[ic_cdk::query]
async fn list() -> Vec<(u64, String, String, u32)> {
    let conn = get_connection().await;

    let mut result: Vec<(u64, String, String, u32)> = vec![];

    let mut rows = conn
        .query("SELECT id, name, data, age FROM person;", ())
        .await
        .unwrap();

    while let Some(row) = rows.next().await.unwrap() {
        let id: u64 = *row.get_value(0).unwrap().as_integer().unwrap() as u64;

        let name: String = row.get_value(1).unwrap().as_text().unwrap().to_owned();
        let data: String = row.get_value(2).unwrap().as_text().unwrap().to_owned();
        let age: u32 = *row.get_value(3).unwrap().as_integer().unwrap() as u32;

        result.push((id, name, data, age));
    }

    result
}

#[ic_cdk::query]
async fn query(sql: String) -> QueryResult {
    let conn = get_connection().await;

    let mut result: Vec<Vec<String>> = Vec::new();

    let mut rows = conn.query(&sql, ()).await.unwrap();

    while let Some(row) = rows.next().await.unwrap() {
        let mut row_data = Vec::new();

        for i in 0..row.column_count() {
            let v: turso::Value = row.get_value(i).unwrap();

            row_data.push(v.as_text().unwrap_or(&"".to_string()).to_string());
        }

        result.push(row_data);
    }

    Ok(result)
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

async fn create_tables() {
    let conn = get_connection().await;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                data  TEXT,
                age   INTEGER
            );",
        (),
    )
    .await
    .unwrap();
}

fn set_pragmas() {
    /* ... */
}

#[ic_cdk::init]
async fn init() {
    mount_memory_files();

    set_pragmas();
    create_tables().await;
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    close_database();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    mount_memory_files();

    set_pragmas();
}

#[derive(CandidType, Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

type QueryResult<T = Vec<Vec<String>>, E = Error> = std::result::Result<T, E>;
