use fake::{Dummy, Fake};
use serde::Serialize;

use super::common::{Commit, CommitUser, DummyRefName, DummyRepository, Repository};

#[derive(Debug, Dummy, Serialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    #[dummy(faker = "DummyRefName")]
    reference: String,
    #[dummy(faker = "DummyRefName")]
    base_ref: String,
    head_commit: Commit,
    #[dummy(faker = "DummyRepository")]
    repository: Repository,
    pusher: CommitUser,
}
