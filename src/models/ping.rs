use fake::{faker::lorem::en::Sentence, Dummy, Fake};
use serde::Serialize;

use super::common::{DummyRepository, Repository, User};

#[derive(Debug, Dummy, Serialize)]
pub struct PingEvent {
    #[dummy(faker = "DummyRepository")]
    repository: Repository,
    sender: Option<User>,
    hook_id: u64,
    #[dummy(faker = "Sentence(1..2)")]
    zen: String,
}
