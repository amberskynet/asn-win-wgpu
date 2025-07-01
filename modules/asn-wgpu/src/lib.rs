pub mod state;

pub use state::{AsnWindow, State};

pub fn get_state() -> State {
    State::default()
}
