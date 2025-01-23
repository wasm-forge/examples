use std::cell::RefCell;

use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

const WASI_MEMORY_ID: MemoryId = MemoryId::new(0);

fn init_wasi() {
    let wasi_memory = MEMORY_MANAGER.with(|m| m.borrow().get(WASI_MEMORY_ID));
    ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], wasi_memory);
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    // Create a directory named "data"
    fs::create_dir_all("data").unwrap();

    // Get the current directory
    let original_dir = env::current_dir().unwrap();
    println!("Starting directory: {:?}", original_dir);

    // Change to the data directory
    env::set_current_dir("data").unwrap();

    // Verify we're in the new directory
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    // Create and write to a file in the data directory
    let file_path = Path::new("example.txt");
    let mut file = File::create(file_path).unwrap();

    // Write some content to the file
    writeln!(file, "Hello from {}.", name).unwrap();
    writeln!(
        file,
        "This is a new line of text, should be there in a file."
    )
    .unwrap();

    file.flush().unwrap();

    fs::read_to_string(file_path).unwrap()
}

#[ic_cdk::init]
fn init() {
    init_wasi();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    init_wasi();
}
