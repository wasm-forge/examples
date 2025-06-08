use std::cell::RefCell;

use candid::CandidType;
use candid::Deserialize;

use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};
use std::rc::Rc;
use std::sync::Arc;

use limbo_core::{Connection, Database, StepResult, IO};

thread_local! {
    static IO: Arc<dyn IO> = Arc::new(limbo_core::PlatformIO::new().unwrap());

    static CONNECTION: RefCell<Option<Rc<Connection>>> = const { RefCell::new(None) };

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

const MOUNTED_MEMORY_ID: u8 = 20;
const DB_FILE_NAME: &str = "db.db3";

fn open_database() {
    IO.with(|io| {
        let db: Arc<Database> = Database::open_file(io.clone(), DB_FILE_NAME, false).unwrap();

        // we only keep connection, it will be enough to keep the database alive
        CONNECTION.with(|con| {
            let mut con = con.borrow_mut();

            *con = Some(db.connect().unwrap());
        });
    });
}

fn close_database() {
    CONNECTION.with(|db| {
        let mut db = db.borrow_mut();
        *db = None;
    });
}

fn escape_sql(input: &str) -> String {
    input.replace("'", "''")
}

#[ic_cdk::update]
fn add(name: String, data: String, age: u32) {
    CONNECTION.with(|con| {
        let con: std::cell::RefMut<'_, Option<Rc<Connection>>> = con.borrow_mut();
        // unwrap Option
        let con: &Rc<Connection> = con.as_ref().unwrap();

        //
        con.execute(format!(
            "INSERT INTO person (name, data, age) VALUES ('{}', '{}', {});",
            escape_sql(&name),
            escape_sql(&data),
            age
        ))
        .unwrap();
    });
}

#[ic_cdk::query]
fn list() -> Vec<(u64, String, String, u32)> {
    CONNECTION.with(|con| {
        let con = con.borrow_mut();
        let con = con.as_ref().unwrap();

        let query_result = con
            .query("SELECT id, name, data, age FROM person;")
            .unwrap();
        let mut result = vec![];

        if let Some(mut query_result) = query_result {
            loop {
                match query_result.step().unwrap() {
                    StepResult::Row => {
                        let row = query_result.row().unwrap();

                        let r: (u64, String, String, u32) = (
                            row.get::<i64>(0).unwrap() as u64,
                            row.get::<String>(1).unwrap(),
                            row.get::<String>(2).unwrap(),
                            row.get::<i64>(3).unwrap() as u32,
                        );

                        result.push(r);
                    }
                    StepResult::IO => {
                        IO.with(|io| {
                            io.run_once().unwrap();
                        });
                    }
                    StepResult::Interrupt => {
                        break;
                    }
                    StepResult::Done => {
                        break;
                    }
                    StepResult::Busy => {
                        break;
                    }
                }
            }
        };

        result
    })
}

#[ic_cdk::query]
fn query(sql: String) -> QueryResult {
    CONNECTION.with(|con| {
        let con = con.borrow_mut();
        let con = con.as_ref().unwrap();

        let mut res: Vec<Vec<String>> = Vec::new();

        let query_result = con.query(&sql).unwrap();

        if let Some(mut query_result) = query_result {
            loop {
                match query_result.step().unwrap() {
                    StepResult::Row => {
                        let mut vec: Vec<String> = Vec::new();

                        // convert all data to string
                        let row = query_result.row().unwrap();

                        for value in row.get_values() {
                            match value {
                                limbo_core::OwnedValue::Null => {
                                    vec.push(String::from("NULL"));
                                }
                                limbo_core::OwnedValue::Integer(i) => {
                                    vec.push(i.to_string());
                                }
                                limbo_core::OwnedValue::Float(f) => {
                                    vec.push(f.to_string());
                                }
                                limbo_core::OwnedValue::Text(text) => {
                                    vec.push(text.to_string());
                                }
                                limbo_core::OwnedValue::Blob(items) => {
                                    vec.push(String::from("<<BLOB>>"));
                                }
                            };
                        }

                        res.push(vec);
                    }
                    StepResult::IO => {
                        IO.with(|io| {
                            io.run_once().unwrap();
                        });
                    }
                    StepResult::Interrupt => {
                        break;
                    }
                    StepResult::Done => {
                        break;
                    }
                    StepResult::Busy => {
                        break;
                    }
                }
            }
        };

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

fn create_tables() {
    CONNECTION.with(|con| {
        let mut con = con.borrow_mut();
        let con = con.as_mut().unwrap();
        con.execute(
            "CREATE TABLE IF NOT EXISTS person (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                data  TEXT,
                age   INTEGER
            );", // empty list of parameters.
        )
        .unwrap();
    });
}

fn set_pragmas() {
    /* ... */
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
