use crate::args::prelude::*;
use crate::error::prelude::*;

pub mod prelude {
    pub use super::action;
    pub use super::Action;
}

pub enum Action {
    Status,
    About,
}

impl Default for Action {
    fn default() -> Self {
        Action::Status
    }
}

pub fn action() -> MyResult<Action> {
    let args = Args::new()?;

    Ok(Action::default())
}
