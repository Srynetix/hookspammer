use chrono::{DateTime, Utc};
use fake::{
    faker::internet::en::{SafeEmail, Username},
    faker::lorem::en::{Sentence, Word},
    Dummy, Fake, Faker,
};
use rand::{prelude::SliceRandom, Rng};
use serde::Serialize;
use strum::{Display, EnumString};

pub struct DummyRefName;
pub struct DummyHex;
pub struct DummyColor;

const HEX_STRING: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

impl Dummy<DummyColor> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DummyColor, rng: &mut R) -> Self {
        let color = (0..6)
            .into_iter()
            .map(|_| HEX_STRING.choose(rng).copied().unwrap())
            .collect::<String>();

        format!("#{color}")
    }
}

impl Dummy<DummyHex> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DummyHex, rng: &mut R) -> Self {
        (0..16)
            .into_iter()
            .map(|_| HEX_STRING.choose(rng).copied().unwrap())
            .collect()
    }
}

impl Dummy<DummyRefName> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DummyRefName, rng: &mut R) -> Self {
        let rand_str: String = Word().fake_with_rng(rng);
        const REF_TYPES: &[&str] = &["branches", "tags"];
        let ref_type = REF_TYPES.choose(rng).unwrap();
        format!("refs/{ref_type}/{rand_str}")
    }
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum GitBackend {
    GitHub,
    GitLab,
}

#[derive(Debug, Clone, Copy, EnumString, Display, Dummy)]
#[strum(serialize_all = "snake_case")]
pub enum EventType {
    CheckSuite,
    IssueComment,
    Ping,
    Push,
    PullRequest,
    Review,
    Unknown,
}

pub struct DummyRepository;

impl Dummy<DummyRepository> for Repository {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DummyRepository, rng: &mut R) -> Self {
        let repo_name: String = Word().fake_with_rng(rng);
        let orga_name: String = Word().fake_with_rng(rng);

        Repository {
            full_name: format!("{orga_name}/{repo_name}"),
            name: repo_name,
            owner: Faker.fake_with_rng(rng),
        }
    }
}

#[derive(Debug, Dummy, Serialize)]
pub struct Repository {
    name: String,
    full_name: String,
    owner: User,
}

#[derive(Debug, Dummy, Serialize)]
pub struct Label {
    #[dummy(faker = "Word()")]
    name: String,
    #[dummy(faker = "DummyColor")]
    color: String,
    #[dummy(faker = "Sentence(1..2)")]
    description: Option<String>,
}

#[derive(Debug, Dummy, Serialize)]
pub struct Application {
    slug: String,
    owner: User,
    #[dummy(faker = "Word()")]
    name: String,
}

#[derive(Debug, Dummy, Serialize)]
pub struct User {
    #[dummy(faker = "Username()")]
    login: String,
    #[dummy(faker = "Word()")]
    name: Option<String>,
    #[dummy(faker = "SafeEmail()")]
    email: Option<String>,
}

#[derive(Debug, Dummy, Serialize)]
pub struct CommitUser {
    #[dummy(faker = "Word()")]
    name: String,
    #[dummy(faker = "SafeEmail()")]
    email: String,
    #[dummy(faker = "Username()")]
    username: Option<String>,
}

#[derive(Debug, Dummy, Serialize)]
pub struct Commit {
    #[dummy(faker = "Sentence(1..2)")]
    message: String,
    timestamp: DateTime<Utc>,
    author: CommitUser,
    committer: CommitUser,
}

#[derive(Debug, Dummy, Serialize)]
pub struct Branch {
    label: Option<String>,
    #[serde(rename = "ref")]
    #[dummy(faker = "DummyRefName")]
    reference: String,
    #[dummy(faker = "DummyHex")]
    sha: String,
    user: Option<User>,
}

#[derive(Debug, Dummy, Serialize)]
pub struct BranchShort {
    #[serde(rename = "ref")]
    #[dummy(faker = "DummyRefName")]
    reference: String,
    #[dummy(faker = "DummyHex")]
    sha: String,
}
