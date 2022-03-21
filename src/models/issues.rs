use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::Serialize;

use super::common::{DummyRepository, Label, Repository, User};

#[derive(Debug, Serialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Debug, Serialize, Dummy)]
#[serde(rename_all = "snake_case")]
pub enum IssueState {
    Open,
    Closed,
}

#[derive(Debug, Serialize, Dummy)]
pub struct Issue {
    number: u64,
    title: String,
    user: User,
    labels: Vec<Label>,
    state: IssueState,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    closed_at: Option<DateTime<Utc>>,
    body: Option<String>,
}

#[derive(Debug, Serialize, Dummy)]
pub struct IssueCommentChangesBody {
    from: String,
}

#[derive(Debug, Serialize, Dummy)]
pub struct IssueCommentChanges {
    body: IssueCommentChangesBody,
}

#[derive(Debug, Serialize, Dummy)]
pub struct IssueComment {
    id: u64,
    user: User,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    body: String,
}

#[derive(Debug, Serialize, Dummy)]
pub struct IssueCommentEvent {
    action: IssueCommentAction,
    changes: Option<IssueCommentChanges>,
    issue: Issue,
    comment: IssueComment,
    #[dummy(faker = "DummyRepository")]
    repository: Repository,
    organization: Option<User>,
    sender: User,
}
