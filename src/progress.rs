use serde::Deserialize;

#[derive(Deserialize)]
pub struct Progress {
    pub(crate) updateId: usize,
}
