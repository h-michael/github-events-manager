use graphql_client;
use query::typedef::*;
use serde;

pub const QUERY : & 'static str = "fragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n\nquery LoginUser {\n  viewer {\n    login\n  }\n}\n\nquery RateLimit {\n  ...RateLimitInfo\n}\n\nquery Repository($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequests($owner: String!, $name: String!, $first: Int = 100, $states: [PullRequestState!] = [OPEN, CLOSED, MERGED]){\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    ullRequests(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n          merged\n          mergedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery Issues($owner: String!, $name: String!, $first: Int = 100, $states: [IssueState!] = [OPEN, CLOSED]){\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    issues(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery WatchingRepositories($first: Int!, $after: String){\n  viewer {\n    watching(after: $after, first: $first) {\n      nodes {\n        id,\n        nameWithOwner,\n        url\n      }\n      pageInfo {\n        hasNextPage,\n        endCursor\n      }\n    }\n  }\n}\n\nquery PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequest {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on CommitCommentThread {\n              id\n              commit_of_comment: commit {\n                ...custom_commit\n              }\n            }\n            ... on PullRequestReview {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              commit_of_pr_review: commit {\n                ...custom_commit\n              }\n              state\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on PullRequestReviewThread {\n              id\n              comments(first: $first) {\n                edges {\n                  node {\n                    id\n                    bodyText\n                    createdAt\n                    updatedAt\n                    lastEditedAt\n                  }\n                }\n              }\n            }\n            ... on PullRequestReviewComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on IssueComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe\n              viewerSubscription\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on MergedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              commit {\n                url\n                commitUrl\n              }\n              isCrossRepository\n              isDirectReference\n              createdAt\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              isCrossRepository\n              url\n              resourcePath\n              willCloseTarget\n              referencedAt\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              currentTitle\n              previousTitle\n              createdAt\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              lockReason\n              createdAt\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on DeployedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DeploymentEnvironmentChangedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on HeadRefDeletedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              headRef {\n                id\n                name\n                prefix\n              }\n              headRefName\n              createdAt\n            }\n            ... on HeadRefRestoredEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on HeadRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              afterCommit {\n                id\n                oid\n              }\n              beforeCommit {\n                id\n                oid\n              }\n              ref {\n                id\n                name\n                prefix\n              }\n              createdAt\n            }\n            ... on BaseRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              afterCommit {\n                id\n                oid\n              }\n              beforeCommit {\n                id\n                oid\n              }\n              ref {\n                id\n                name\n                prefix\n              }\n              createdAt\n            }\n            # ... on ReviewRequestedEvent {\n            #   id\n            #   actor {\n            #     __typename\n            #     ...custom_actor\n            #   }\n            #   requestedReviewer {\n            #     ...on User {\n            #       id\n            #       userName: name\n            #       userLogin: login\n            #     }\n            #     ...on Team {\n            #       id\n            #       teamName: name\n            #     }\n            #   }\n            #   createdAt\n            # }\n            # ... on ReviewRequestRemovedEvent {\n            #   id\n            #   actor {\n            #     __typename\n            #     ...custom_actor\n            #   }\n            #   requestedReviewer {\n            #     ...on User {\n            #       id\n            #       userName: name\n            #       userLogin: login\n            #     }\n            #     ...on Team {\n            #       id\n            #       teamName: name\n            #     }\n            #   }\n            #   createdAt\n            # }\n            ... on ReviewDismissedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              message\n              resourcePath\n              url\n              previousReviewState\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestReview($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequestReview {\n      comments(first: $first) {\n        nodes {\n          id\n          bodyText\n          viewerDidAuthor\n        }\n      }\n    }\n  }\n}\n\nquery IssueTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              bodyText\n              createdAt\n              lastEditedAt\n              publishedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe\n              viewerSubscription\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              commit {\n                url\n                commitUrl\n              }\n              isCrossRepository\n              isDirectReference\n              createdAt\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              isCrossRepository\n              url\n              resourcePath\n              willCloseTarget\n              referencedAt\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              currentTitle\n              previousTitle\n              createdAt\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              lockReason\n              createdAt\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n" ;
pub const OPERATION_NAME: &'static str = "PullRequestTimelineItems";
#[allow(dead_code)]
type Boolean = bool;
#[allow(dead_code)]
type Float = f64;
#[allow(dead_code)]
type Int = i64;
#[allow(dead_code)]
type ID = String;
#[derive(Debug)]
pub enum LockReason {
    OFF_TOPIC,
    RESOLVED,
    SPAM,
    TOO_HEATED,
    Other(String),
}
impl ::serde::Serialize for LockReason {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            LockReason::OFF_TOPIC => "OFF_TOPIC",
            LockReason::RESOLVED => "RESOLVED",
            LockReason::SPAM => "SPAM",
            LockReason::TOO_HEATED => "TOO_HEATED",
            LockReason::Other(ref s) => &s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for LockReason {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String>::deserialize(deserializer)?;
        match s.as_str() {
            "OFF_TOPIC" => Ok(LockReason::OFF_TOPIC),
            "RESOLVED" => Ok(LockReason::RESOLVED),
            "SPAM" => Ok(LockReason::SPAM),
            "TOO_HEATED" => Ok(LockReason::TOO_HEATED),
            _ => Ok(LockReason::Other(s)),
        }
    }
}
#[derive(Debug)]
pub enum PullRequestReviewState {
    APPROVED,
    CHANGES_REQUESTED,
    COMMENTED,
    DISMISSED,
    PENDING,
    Other(String),
}
impl ::serde::Serialize for PullRequestReviewState {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            PullRequestReviewState::APPROVED => "APPROVED",
            PullRequestReviewState::CHANGES_REQUESTED => "CHANGES_REQUESTED",
            PullRequestReviewState::COMMENTED => "COMMENTED",
            PullRequestReviewState::DISMISSED => "DISMISSED",
            PullRequestReviewState::PENDING => "PENDING",
            PullRequestReviewState::Other(ref s) => &s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for PullRequestReviewState {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String>::deserialize(deserializer)?;
        match s.as_str() {
            "APPROVED" => Ok(PullRequestReviewState::APPROVED),
            "CHANGES_REQUESTED" => Ok(PullRequestReviewState::CHANGES_REQUESTED),
            "COMMENTED" => Ok(PullRequestReviewState::COMMENTED),
            "DISMISSED" => Ok(PullRequestReviewState::DISMISSED),
            "PENDING" => Ok(PullRequestReviewState::PENDING),
            _ => Ok(PullRequestReviewState::Other(s)),
        }
    }
}
#[derive(Debug)]
pub enum SubscriptionState {
    IGNORED,
    SUBSCRIBED,
    UNSUBSCRIBED,
    Other(String),
}
impl ::serde::Serialize for SubscriptionState {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(match *self {
            SubscriptionState::IGNORED => "IGNORED",
            SubscriptionState::SUBSCRIBED => "SUBSCRIBED",
            SubscriptionState::UNSUBSCRIBED => "UNSUBSCRIBED",
            SubscriptionState::Other(ref s) => &s,
        })
    }
}
impl<'de> ::serde::Deserialize<'de> for SubscriptionState {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <String>::deserialize(deserializer)?;
        match s.as_str() {
            "IGNORED" => Ok(SubscriptionState::IGNORED),
            "SUBSCRIBED" => Ok(SubscriptionState::SUBSCRIBED),
            "UNSUBSCRIBED" => Ok(SubscriptionState::UNSUBSCRIBED),
            _ => Ok(SubscriptionState::Other(s)),
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct RateLimitInfo {
    #[serde(rename = "rateLimit")]
    pub rate_limit: Option<RateLimitInfoRateLimit>,
}
#[derive(Deserialize, Debug)]
pub struct RateLimitInfoRateLimit {
    pub cost: Int,
    pub limit: Int,
    pub remaining: Int,
    #[serde(rename = "resetAt")]
    pub reset_at: DateTime,
}
#[derive(Deserialize, Debug)]
pub struct custom_actor {}
#[derive(Deserialize, Debug)]
pub struct custom_commit {
    pub id: ID,
    pub oid: GitObjectID,
    #[serde(rename = "messageBody")]
    pub message_body: String,
    #[serde(rename = "messageHeadline")]
    pub message_headline: String,
    #[serde(rename = "commitUrl")]
    pub commit_url: URI,
    #[serde(rename = "committedDate")]
    pub committed_date: DateTime,
    #[serde(rename = "pushedDate")]
    pub pushed_date: Option<DateTime>,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestAuthorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestAuthor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestAuthorOn,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestEditorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestEditor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestEditorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommit {
    #[serde(flatten)]
    pub custom_commit: custom_commit,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThreadCommitOfComment
{
    #[serde(flatten)]
    pub custom_commit: custom_commit,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThread { pub id : ID , pub commit_of_comment : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThreadCommitOfComment , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewCommitOfPrReview
{
    #[serde(flatten)]
    pub custom_commit: custom_commit,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReview { pub id : ID , pub author : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthor > , pub commit_of_pr_review : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewCommitOfPrReview > , pub state : PullRequestReviewState , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , # [ serde ( rename = "updatedAt" ) ] pub updated_at : DateTime , # [ serde ( rename = "lastEditedAt" ) ] pub last_edited_at : Option < DateTime > , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdgesNode
{
    pub id: ID,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "lastEditedAt")]
    pub last_edited_at: Option<DateTime>,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdges { pub node : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdgesNode > , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadComments { pub edges : Option < Vec < Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdges > > > , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThread { pub id : ID , pub comments : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadComments , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewComment
{
    pub id: ID,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "lastEditedAt")]
    pub last_edited_at: Option<DateTime>,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnIssueComment {
    pub id: ID,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime,
    #[serde(rename = "lastEditedAt")]
    pub last_edited_at: Option<DateTime>,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActor,
    >,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActor,
    >,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribableOn {
    Repository,
    Commit,
    PullRequest,
    Issue,
    Team,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribable {
    pub id: ID,
    #[serde(rename = "viewerCanSubscribe")]
    pub viewer_can_subscribe: Boolean,
    #[serde(rename = "viewerSubscription")]
    pub viewer_subscription: Option<SubscriptionState>,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribableOn,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActor,
    >,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActor,
    >,
    pub url: URI,
    #[serde(rename = "resourcePath")]
    pub resource_path: URI,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventCommit {
    pub url: URI,
    #[serde(rename = "commitUrl")]
    pub commit_url: URI,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActor,
    >,
    pub commit: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventCommit,
    >,
    #[serde(rename = "isCrossRepository")]
    pub is_cross_repository: Boolean,
    #[serde(rename = "isDirectReference")]
    pub is_direct_reference: Boolean,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActor > , # [ serde ( rename = "isCrossRepository" ) ] pub is_cross_repository : Boolean , pub url : URI , # [ serde ( rename = "resourcePath" ) ] pub resource_path : URI , # [ serde ( rename = "willCloseTarget" ) ] pub will_close_target : Boolean , # [ serde ( rename = "referencedAt" ) ] pub referenced_at : DateTime , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventUser {
    pub name: Option<String>,
    pub login: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActor,
    >,
    pub user: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventUser,
    >,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventUser {
    pub name: Option<String>,
    pub login: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActor,
    >,
    pub user: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventUser,
    >,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventLabel {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActor,
    >,
    pub label:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventLabel,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventLabel {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActor,
    >,
    pub label:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventLabel,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActor,
    >,
    #[serde(rename = "milestoneTitle")]
    pub milestone_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActor,
    >,
    #[serde(rename = "milestoneTitle")]
    pub milestone_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActor,
    >,
    #[serde(rename = "currentTitle")]
    pub current_title: String,
    #[serde(rename = "previousTitle")]
    pub previous_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActor,
    >,
    #[serde(rename = "lockReason")]
    pub lock_reason: Option<LockReason>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActor,
    >,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActorOn {
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on:
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEvent {
    pub id: ID,
    pub actor: Option<
        RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActor,
    >,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActor > , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventHeadRef
{
    pub id: ID,
    pub name: String,
    pub prefix: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActor > , # [ serde ( rename = "headRef" ) ] pub head_ref : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventHeadRef > , # [ serde ( rename = "headRefName" ) ] pub head_ref_name : String , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActor > , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventAfterCommit
{
    pub id: ID,
    pub oid: GitObjectID,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventBeforeCommit
{
    pub id: ID,
    pub oid: GitObjectID,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventRef
{
    pub id: ID,
    pub name: String,
    pub prefix: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActor > , # [ serde ( rename = "afterCommit" ) ] pub after_commit : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventAfterCommit > , # [ serde ( rename = "beforeCommit" ) ] pub before_commit : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventBeforeCommit > , # [ serde ( rename = "ref" ) ] pub ref_ : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventRef > , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventAfterCommit
{
    pub id: ID,
    pub oid: GitObjectID,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventBeforeCommit
{
    pub id: ID,
    pub oid: GitObjectID,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventRef
{
    pub id: ID,
    pub name: String,
    pub prefix: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActor > , # [ serde ( rename = "afterCommit" ) ] pub after_commit : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventAfterCommit > , # [ serde ( rename = "beforeCommit" ) ] pub before_commit : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventBeforeCommit > , # [ serde ( rename = "ref" ) ] pub ref_ : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventRef > , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActorOn
{
    Organization,
    Bot,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActor > , pub message : String , # [ serde ( rename = "resourcePath" ) ] pub resource_path : URI , pub url : URI , # [ serde ( rename = "previousReviewState" ) ] pub previous_review_state : PullRequestReviewState , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNode {
     Commit ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommit ) , CommitCommentThread ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThread ) , PullRequestReview ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReview ) , PullRequestReviewThread ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThread ) , PullRequestReviewComment ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewComment ) , IssueComment ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnIssueComment ) , ClosedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEvent ) , ReopenedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEvent ) , Subscribable ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribable ) , UnsubscribedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEvent ) , MergedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEvent ) , ReferencedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEvent ) , CrossReferencedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEvent ) , AssignedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEvent ) , UnassignedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEvent ) , LabeledEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEvent ) , UnlabeledEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEvent ) , MilestonedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEvent ) , DemilestonedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEvent ) , RenamedTitleEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEvent ) , LockedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEvent ) , UnlockedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEvent ) , DeployedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEvent ) , DeploymentEnvironmentChangedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEvent ) , HeadRefDeletedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEvent ) , HeadRefRestoredEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEvent ) , HeadRefForcePushedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEvent ) , BaseRefForcePushedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEvent ) , ReviewDismissedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEvent ) , ReviewRequestRemovedEvent , ReviewRequestedEvent , SubscribedEvent }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdges {
    pub node: Option<RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNode>,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelinePageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimeline {
    pub edges: Option<Vec<Option<RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdges>>>,
    #[serde(rename = "pageInfo")]
    pub page_info: RustPullRequestTimelineItemsNodeOnPullRequestTimelinePageInfo,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequest {
    pub id: ID,
    pub author: Option<RustPullRequestTimelineItemsNodeOnPullRequestAuthor>,
    pub editor: Option<RustPullRequestTimelineItemsNodeOnPullRequestEditor>,
    pub title: String,
    pub body: String,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    pub timeline: RustPullRequestTimelineItemsNodeOnPullRequestTimeline,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOn {
    PullRequest(RustPullRequestTimelineItemsNodeOnPullRequest),
    Release,
    BaseRefChangedEvent,
    Push,
    StatusContext,
    UnsubscribedEvent,
    PullRequestReviewThread,
    RepositoryTopic,
    License,
    LabeledEvent,
    User,
    AssignedEvent,
    Gist,
    IssueComment,
    OrganizationIdentityProvider,
    Repository,
    ReferencedEvent,
    MentionedEvent,
    ReviewDismissedEvent,
    SubscribedEvent,
    HeadRefRestoredEvent,
    CommitComment,
    PullRequestReview,
    UnlockedEvent,
    UnlabeledEvent,
    DemilestonedEvent,
    ReviewRequestRemovedEvent,
    MovedColumnsInProjectEvent,
    DeployKey,
    MarketplaceCategory,
    ClosedEvent,
    App,
    RemovedFromProjectEvent,
    Project,
    Organization,
    CrossReferencedEvent,
    MarketplaceListing,
    CommentDeletedEvent,
    DeployedEvent,
    ReleaseAsset,
    PullRequestCommit,
    LockedEvent,
    ProjectCard,
    Tag,
    MilestonedEvent,
    Ref,
    Reaction,
    ExternalIdentity,
    AddedToProjectEvent,
    Topic,
    CheckSuite,
    CommitCommentThread,
    GistComment,
    Team,
    Tree,
    HeadRefForcePushedEvent,
    UnassignedEvent,
    Blob,
    PullRequestCommitCommentThread,
    RepositoryInvitation,
    ProjectColumn,
    Deployment,
    Issue,
    Commit,
    ReopenedEvent,
    DeploymentStatus,
    CheckRun,
    DeploymentEnvironmentChangedEvent,
    HeadRefDeletedEvent,
    MergedEvent,
    Label,
    BaseRefForcePushedEvent,
    UserContentEdit,
    ReviewDismissalAllowance,
    ConvertedNoteToIssueEvent,
    PullRequestReviewComment,
    PushAllowance,
    Status,
    Bot,
    ReviewRequestedEvent,
    RenamedTitleEvent,
    PublicKey,
    Language,
    OrganizationInvitation,
    Milestone,
    ProtectedBranch,
    ReviewRequest,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNode {
    #[serde(flatten)]
    pub on: RustPullRequestTimelineItemsNodeOn,
}
#[derive(Serialize, Debug)]
pub struct Variables {
    pub id: ID,
    pub first: Option<Int>,
}
impl Variables {
    pub fn default_first() -> Option<Int> {
        Some(100i64)
    }
}
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub node: Option<RustPullRequestTimelineItemsNode>,
    #[serde(flatten)]
    pub rate_limit_info: RateLimitInfo,
}

pub struct PullRequestTimelineItems;

impl<'de> graphql_client::GraphQLQuery<'de> for PullRequestTimelineItems {
    type Variables = Variables;
    type ResponseData = ResponseData;
    fn build_query(variables: Self::Variables) -> graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
