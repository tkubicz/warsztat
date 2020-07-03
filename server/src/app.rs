use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::delay_for;
use std::time::Duration;

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

    pub async fn getCounter(self: &Arc<Self>) -> u64 {
        let guard = self.state.lock().await;
        (*guard).counter
    }

    pub async fn incrementCounter(self: &Arc<Self>) {
        let mut guard = self.state.lock().await;
        delay_for(Duration::from_millis(100)).await;            //Sztuczne opóźnienie
        (*guard).counter += 1;
    }
}