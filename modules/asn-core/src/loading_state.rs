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

impl<S> LoadingState<S> {
    pub fn load(&mut self) -> LoadingState<S> {
        let new_state = std::mem::replace(self, Self::Zero);
        match new_state {
            Self::Empty(r) => Self::Loaded(r),
            loaded @ Self::Loaded(_) => loaded,
            Self::Zero => panic!("State is zero"),
        }
    }
}
