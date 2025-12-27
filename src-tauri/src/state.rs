use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct CaffeinateState {
    running: AtomicBool,
}

impl CaffeinateState {
    pub fn set_running(&self, is_running: bool) {
        self.running.store(is_running, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}
