use std::fmt;

#[derive(Debug)]
pub enum LoadingState<S> {
    Zero,
    Empty(S),
    Loaded(S),
}

impl<S> fmt::Display for LoadingState<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadingState::Zero => write!(f, "LoadingState::Zero"),
            LoadingState::Empty(_) => write!(f, "LoadingState::Empty"),
            LoadingState::Loaded(_) => write!(f, "LoadingState::Loaded"),
        }
    }
}

pub fn set_state_loaded<S>(state: &mut LoadingState<S>) -> LoadingState<S> {
    let new_state = std::mem::replace(state, LoadingState::Zero);
    match new_state {
        LoadingState::Empty(r) => LoadingState::Loaded(r),
        loaded @ LoadingState::Loaded(_) => loaded,
        LoadingState::Zero => panic!("State is zero"),
    }
}
