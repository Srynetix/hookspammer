use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::Serialize;

use super::common::{Application, BranchShort, DummyRepository, Repository, User};

#[derive(Debug, Dummy, Serialize)]
pub struct PullRequestShort {
    pub number: u64,
    pub head: BranchShort,
    pub base: BranchShort,
}

#[derive(Debug, Dummy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckSuiteAction {
    Completed,
    Requested,
    Rerequested,
}

#[derive(Debug, Dummy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    Completed,
    InProgress,
    Queued,
    Requested,
}

#[derive(Debug, Dummy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckConclusion {
    ActionRequired,
    Cancelled,
    Failure,
    Neutral,
    Skipped,
    Stale,
    Success,
    TimedOut,
}

#[derive(Debug, Dummy, Serialize)]
pub struct CheckSuite {
    id: u64,
    head_branch: String,
    head_sha: String,
    status: CheckStatus,
    conclusion: Option<CheckConclusion>,
    pull_requests: Vec<PullRequestShort>,
    app: Application,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Dummy, Serialize)]
pub struct CheckSuiteEvent {
    action: CheckSuiteAction,
    check_suite: CheckSuite,
    #[dummy(faker = "DummyRepository")]
    repository: Repository,
    organization: Option<User>,
    sender: User,
}
