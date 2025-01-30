use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use ic_stable_structures::VectorMemory;

// initialize new vector memory from a vector
fn new_vector_memory(v: Vec<u8>) -> VectorMemory {
    use std::{cell::RefCell, rc::Rc};
    Rc::new(RefCell::new(v))
}

thread_local! {
    static MEMORY: VectorMemory = new_vector_memory(Vec::new());
}

fn init_wasi() {
    MEMORY.with(|m| {
        ic_wasi_polyfill::init_with_memory(&[0u8; 32], &[], m.clone());
    });
}

#[ic_cdk::init]
fn init() {
    init_wasi();
}

#[ic_cdk::update]
pub fn fs_size() -> u64 {
    MEMORY.with(|m| {
        // get fs memory
        let v = m.borrow();

        // return the current size of the memory
        v.len() as u64
    })
}

#[ic_cdk::update]
pub fn write_file(file_name: String, content: String) {
    let path = Path::new(&file_name);

    // Ensure parent directories exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Create and write to the file
    let mut file = File::create(&file_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[ic_cdk::update]
pub fn read_file(file_name: String) -> String {
    fs::read_to_string(&file_name).unwrap()
}
