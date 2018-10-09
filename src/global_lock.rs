use std::sync::{Mutex, MutexGuard};

// fight global with global
static mut LOCK: Option<Mutex<()>> = None;

fn init_lock() {
    unsafe {
        LOCK = Some(Mutex::new(()));
    }
}

fn acquire_lock() -> MutexGuard<'static, ()> {
    unsafe {
    	LOCK.as_ref().unwrap().lock().unwrap()
	}
}	