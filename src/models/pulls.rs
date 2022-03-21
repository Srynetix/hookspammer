use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::Serialize;

use super::common::{Branch, DummyRepository, Label, Repository, User};

#[derive(Debug, Serialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Assigned,
    Closed,
    ConvertedToDraft,
    Edited,
    Labeled,
    Locked,
    Opened,
    Reopened,
    ReadyForReview,
    ReviewRequested,
    ReviewRequestRemoved,
    Synchronize,
    Unassigned,
    Unlabeled,
    Unlocked,
}

#[derive(Debug, Serialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Serialize, Dummy)]
pub struct PullRequest {
    number: u64,
    state: PullRequestState,
    locked: bool,
    title: String,
    user: User,
    body: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
    merged_at: Option<DateTime<Utc>>,
    requested_reviewers: Vec<User>,
    labels: Vec<Label>,
    draft: bool,
    head: Branch,
    base: Branch,
    merged: Option<bool>,
    mergeable: Option<bool>,
    rebaseable: Option<bool>,
}

#[derive(Debug, Serialize, Dummy)]
pub struct PullRequestEvent {
    action: PullRequestAction,
    number: u64,
    pull_request: PullRequest,
    label: Option<Label>,
    requested_reviewer: Option<User>,
    #[dummy(faker = "DummyRepository")]
    repository: Repository,
    organization: Option<User>,
    sender: User,
}
