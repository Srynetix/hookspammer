use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::Serialize;

use super::{
    common::{DummyRepository, Repository, User},
    pulls::PullRequest,
};

#[derive(Debug, Serialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

#[derive(Debug, Serialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    Approved,
    ChangesRequested,
    Commented,
    Dismissed,
    Pending,
}

#[derive(Debug, Serialize, Dummy)]
pub struct Review {
    user: User,
    submitted_at: DateTime<Utc>,
    state: ReviewState,
}

#[derive(Debug, Serialize, Dummy)]
pub struct ReviewEvent {
    action: ReviewAction,
    review: Review,
    pull_request: PullRequest,
    #[dummy(faker = "DummyRepository")]
    repository: Repository,
    organization: Option<User>,
    sender: User,
}
