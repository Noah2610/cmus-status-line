pub mod prelude {
    pub use super::action;
    pub use super::Action;
}

pub enum Action {
    PrintStatus,
    PrintAbout,
}

impl Default for Action {
    fn default() -> Self {
        Action::PrintStatus
    }
}

pub fn action() -> Action {
    Action::default()
}
