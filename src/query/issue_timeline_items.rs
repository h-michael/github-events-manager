use graphql_client;
use query::typedef::*;
use serde;

pub const QUERY : & 'static str = "fragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n\nquery LoginUser {\n  viewer {\n    login\n  }\n  ...RateLimitInfo\n}\n\nquery RateLimit {\n  ...RateLimitInfo\n}\n\nquery Repository($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequests($owner: String!, $name: String!, $first: Int = 100, $states: [PullRequestState!] = [OPEN, CLOSED, MERGED]) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    pullRequests(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n          merged\n          mergedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery Issues($owner: String!, $name: String!, $first: Int = 100, $states: [IssueState!] = [OPEN, CLOSED]){\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    issues(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery WatchingRepositories($first: Int!, $after: String){\n  viewer {\n    watching(after: $after, first: $first) {\n      nodes {\n        id,\n        nameWithOwner,\n        url\n      }\n      pageInfo {\n        hasNextPage,\n        endCursor\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequest {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on CommitCommentThread {\n              id\n              commit_of_comment: commit {\n                ...custom_commit\n              }\n            }\n            ... on PullRequestReview {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              commit_of_pr_review: commit {\n                ...custom_commit\n              }\n              state\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on PullRequestReviewThread {\n              id\n              comments(first: $first) {\n                edges {\n                  node {\n                    id\n                    bodyText\n                    createdAt\n                    updatedAt\n                    lastEditedAt\n                  }\n                }\n              }\n            }\n            ... on PullRequestReviewComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on IssueComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe\n              viewerSubscription\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on MergedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              commit {\n                url\n                commitUrl\n              }\n              isCrossRepository\n              isDirectReference\n              createdAt\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              isCrossRepository\n              url\n              resourcePath\n              willCloseTarget\n              referencedAt\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              currentTitle\n              previousTitle\n              createdAt\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              lockReason\n              createdAt\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on DeployedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DeploymentEnvironmentChangedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on HeadRefDeletedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              headRef {\n                id\n                name\n                prefix\n              }\n              headRefName\n              createdAt\n            }\n            ... on HeadRefRestoredEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on HeadRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              afterCommit {\n                id\n                oid\n              }\n              beforeCommit {\n                id\n                oid\n              }\n              ref {\n                id\n                name\n                prefix\n              }\n              createdAt\n            }\n            ... on BaseRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              afterCommit {\n                id\n                oid\n              }\n              beforeCommit {\n                id\n                oid\n              }\n              ref {\n                id\n                name\n                prefix\n              }\n              createdAt\n            }\n            # ... on ReviewRequestedEvent {\n            #   id\n            #   actor {\n            #     __typename\n            #     ...custom_actor\n            #   }\n            #   requestedReviewer {\n            #     ...on User {\n            #       id\n            #       userName: name\n            #       userLogin: login\n            #     }\n            #     ...on Team {\n            #       id\n            #       teamName: name\n            #     }\n            #   }\n            #   createdAt\n            # }\n            # ... on ReviewRequestRemovedEvent {\n            #   id\n            #   actor {\n            #     __typename\n            #     ...custom_actor\n            #   }\n            #   requestedReviewer {\n            #     ...on User {\n            #       id\n            #       userName: name\n            #       userLogin: login\n            #     }\n            #     ...on Team {\n            #       id\n            #       teamName: name\n            #     }\n            #   }\n            #   createdAt\n            # }\n            ... on ReviewDismissedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              message\n              resourcePath\n              url\n              previousReviewState\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestReview($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequestReview {\n      comments(first: $first) {\n        nodes {\n          id\n          bodyText\n          viewerDidAuthor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery IssueTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              bodyText\n              createdAt\n              lastEditedAt\n              publishedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe\n              viewerSubscription\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              commit {\n                url\n                commitUrl\n              }\n              isCrossRepository\n              isDirectReference\n              createdAt\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              isCrossRepository\n              url\n              resourcePath\n              willCloseTarget\n              referencedAt\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              currentTitle\n              previousTitle\n              createdAt\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              lockReason\n              createdAt\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n" ;
pub const OPERATION_NAME: &'static str = "IssueTimelineItems";
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
pub enum RustIssueTimelineItemsNodeOnIssueAuthorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueAuthor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueAuthorOn,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueEditorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueEditor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueEditorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCommit {
    #[serde(flatten)]
    pub custom_commit: custom_commit,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueCommentAuthorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueCommentAuthor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueCommentAuthorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueComment {
    pub id: ID,
    pub author: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueCommentAuthor>,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "lastEditedAt")]
    pub last_edited_at: Option<DateTime>,
    #[serde(rename = "publishedAt")]
    pub published_at: Option<DateTime>,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActor>,
    pub url: URI,
    #[serde(rename = "resourcePath")]
    pub resource_path: URI,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActor>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribableOn {
    Issue,
    Repository,
    Team,
    PullRequest,
    Commit,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribable {
    pub id: ID,
    #[serde(rename = "viewerCanSubscribe")]
    pub viewer_can_subscribe: Boolean,
    #[serde(rename = "viewerSubscription")]
    pub viewer_subscription: Option<SubscriptionState>,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribableOn,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActor>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventCommit {
    pub url: URI,
    #[serde(rename = "commitUrl")]
    pub commit_url: URI,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActor>,
    pub commit: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventCommit>,
    #[serde(rename = "isCrossRepository")]
    pub is_cross_repository: Boolean,
    #[serde(rename = "isDirectReference")]
    pub is_direct_reference: Boolean,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEvent {
    pub id: ID,
    pub actor:
        Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActor>,
    #[serde(rename = "isCrossRepository")]
    pub is_cross_repository: Boolean,
    pub url: URI,
    #[serde(rename = "resourcePath")]
    pub resource_path: URI,
    #[serde(rename = "willCloseTarget")]
    pub will_close_target: Boolean,
    #[serde(rename = "referencedAt")]
    pub referenced_at: DateTime,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventUser {
    pub name: Option<String>,
    pub login: String,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActor>,
    pub user: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventUser>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventUser {
    pub name: Option<String>,
    pub login: String,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActor>,
    pub user: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventUser>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventLabel {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActor>,
    pub label: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventLabel,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventLabel {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActor>,
    pub label: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventLabel,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActor>,
    #[serde(rename = "milestoneTitle")]
    pub milestone_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActor>,
    #[serde(rename = "milestoneTitle")]
    pub milestone_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActor>,
    #[serde(rename = "currentTitle")]
    pub current_title: String,
    #[serde(rename = "previousTitle")]
    pub previous_title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActor>,
    #[serde(rename = "lockReason")]
    pub lock_reason: Option<LockReason>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActorOn {
    Bot,
    Organization,
    User,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActor {
    #[serde(flatten)]
    pub custom_actor: custom_actor,
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActorOn,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEvent {
    pub id: ID,
    pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActor>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNode {
    Commit(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCommit),
    IssueComment(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueComment),
    ClosedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEvent),
    ReopenedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEvent),
    Subscribable(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribable),
    UnsubscribedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEvent),
    ReferencedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEvent),
    CrossReferencedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEvent),
    AssignedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEvent),
    UnassignedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEvent),
    LabeledEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEvent),
    UnlabeledEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEvent),
    MilestonedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEvent),
    DemilestonedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEvent),
    RenamedTitleEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEvent),
    LockedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEvent),
    UnlockedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEvent),
    SubscribedEvent,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdges {
    pub node: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNode>,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimelinePageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssueTimeline {
    pub edges: Option<Vec<Option<RustIssueTimelineItemsNodeOnIssueTimelineEdges>>>,
    #[serde(rename = "pageInfo")]
    pub page_info: RustIssueTimelineItemsNodeOnIssueTimelinePageInfo,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNodeOnIssue {
    pub id: ID,
    pub author: Option<RustIssueTimelineItemsNodeOnIssueAuthor>,
    pub editor: Option<RustIssueTimelineItemsNodeOnIssueEditor>,
    pub title: String,
    pub body: String,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    pub timeline: RustIssueTimelineItemsNodeOnIssueTimeline,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustIssueTimelineItemsNodeOn {
    Issue(RustIssueTimelineItemsNodeOnIssue),
    ClosedEvent,
    Blob,
    Team,
    Commit,
    ConvertedNoteToIssueEvent,
    MarketplaceCategory,
    PullRequestCommit,
    CommitCommentThread,
    ReviewRequestedEvent,
    ReviewRequestRemovedEvent,
    ReviewRequest,
    ReviewDismissedEvent,
    Repository,
    GistComment,
    ExternalIdentity,
    HeadRefForcePushedEvent,
    CommentDeletedEvent,
    DeployedEvent,
    Status,
    Reaction,
    ReviewDismissalAllowance,
    UserContentEdit,
    User,
    MentionedEvent,
    Topic,
    AddedToProjectEvent,
    Tag,
    App,
    LockedEvent,
    License,
    DemilestonedEvent,
    OrganizationIdentityProvider,
    ReferencedEvent,
    RenamedTitleEvent,
    RepositoryTopic,
    PublicKey,
    ProjectColumn,
    Tree,
    UnlockedEvent,
    Organization,
    PullRequestCommitCommentThread,
    Release,
    IssueComment,
    PullRequestReview,
    PullRequest,
    CheckSuite,
    DeploymentEnvironmentChangedEvent,
    Bot,
    RemovedFromProjectEvent,
    MergedEvent,
    ProjectCard,
    StatusContext,
    HeadRefRestoredEvent,
    PullRequestReviewComment,
    SubscribedEvent,
    UnsubscribedEvent,
    HeadRefDeletedEvent,
    ReopenedEvent,
    BaseRefForcePushedEvent,
    CommitComment,
    LabeledEvent,
    Label,
    UnlabeledEvent,
    Push,
    DeploymentStatus,
    Milestone,
    MilestonedEvent,
    OrganizationInvitation,
    AssignedEvent,
    UnassignedEvent,
    ReleaseAsset,
    RepositoryInvitation,
    CrossReferencedEvent,
    BaseRefChangedEvent,
    Deployment,
    Ref,
    MarketplaceListing,
    Project,
    PullRequestReviewThread,
    Language,
    MovedColumnsInProjectEvent,
    Gist,
    DeployKey,
    PushAllowance,
    ProtectedBranch,
    CheckRun,
}
#[derive(Deserialize, Debug)]
pub struct RustIssueTimelineItemsNode {
    #[serde(flatten)]
    pub on: RustIssueTimelineItemsNodeOn,
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
    pub node: Option<RustIssueTimelineItemsNode>,
    #[serde(flatten)]
    pub rate_limit_info: RateLimitInfo,
}

pub struct IssueTimelineItems;

impl<'de> ::graphql_client::GraphQLQuery<'de> for IssueTimelineItems {
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
