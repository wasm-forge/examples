use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};
use std::cell::RefCell;

thread_local! {
    // The memory manager enables multiple virtual memories in one.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    ic_cdk::api::debug_print(format!("Hello from IC debugger: {name}"));
    println!("Hello from WASI: {name}");
    format!("Hello, {name}!")
}

#[ic_cdk::init]
fn init() {
    MEMORY_MANAGER.with(|m| {
        let m = m.borrow();
        ic_wasi_polyfill::init_with_memory_manager(&[0u8; 32], &[], &m, 200..210);
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    MEMORY_MANAGER.with(|m| {
        let m = m.borrow();
        ic_wasi_polyfill::init_with_memory_manager(&[0u8; 32], &[], &m, 200..210);
    });
}
