use std::sync::Arc;
use tokio::sync::Mutex;

struct AppStateInner {
    counter: u64,
}

impl AppStateInner {
    pub fn new() -> AppStateInner {
        AppStateInner {
            counter: 0,
        }
    }
}

pub struct AppState {
    state: Mutex<AppStateInner>,
}

impl AppState {
    pub fn new() -> Arc<AppState> {
        Arc::new(AppState {
            state: Mutex::new(AppStateInner::new())
        })
    } 
}