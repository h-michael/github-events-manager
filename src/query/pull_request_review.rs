#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::query::typedef::*;
use graphql_client;

pub const QUERY : & 'static str = "fragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n\nquery LoginUser {\n  viewer {\n    login\n  }\n  ...RateLimitInfo\n}\n\nquery RateLimit {\n  ...RateLimitInfo\n}\n\nquery Repository($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequests($owner: String!, $name: String!, $first: Int = 100, $states: [PullRequestState!] = [OPEN, CLOSED, MERGED]) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    pullRequests(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n          merged\n          mergedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery Issues($owner: String!, $name: String!, $first: Int = 100, $states: [IssueState!] = [OPEN, CLOSED]){\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    issues(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery WatchingRepositories($first: Int!, $after: String){\n  viewer {\n    watching(after: $after, first: $first) {\n      nodes {\n        id,\n        nameWithOwner,\n        url\n      }\n      pageInfo {\n        hasNextPage,\n        endCursor\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequest {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on CommitCommentThread {\n              id\n              commit_of_comment: commit {\n                ...custom_commit\n              }\n            }\n            ... on PullRequestReview {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              commit_of_pr_review: commit {\n                ...custom_commit\n              }\n              state\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on PullRequestReviewThread {\n              id\n              comments(first: $first) {\n                edges {\n                  node {\n                    id\n                    bodyText\n                    createdAt\n                    updatedAt\n                    lastEditedAt\n                  }\n                }\n              }\n            }\n            ... on PullRequestReviewComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on IssueComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe\n              viewerSubscription\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on MergedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              commit {\n                url\n                commitUrl\n              }\n              isCrossRepository\n              isDirectReference\n              createdAt\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              isCrossRepository\n              url\n              resourcePath\n              willCloseTarget\n              referencedAt\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              currentTitle\n              previousTitle\n              createdAt\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              lockReason\n              createdAt\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on DeployedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DeploymentEnvironmentChangedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on HeadRefDeletedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              headRef {\n                id\n                name\n                prefix\n              }\n              headRefName\n              createdAt\n            }\n            ... on HeadRefRestoredEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on HeadRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              afterCommit {\n                id\n                oid\n              }\n              beforeCommit {\n                id\n                oid\n              }\n              ref {\n                id\n                name\n                prefix\n              }\n              createdAt\n            }\n            ... on BaseRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              afterCommit {\n                id\n                oid\n              }\n              beforeCommit {\n                id\n                oid\n              }\n              ref {\n                id\n                name\n                prefix\n              }\n              createdAt\n            }\n            # ... on ReviewRequestedEvent {\n            #   id\n            #   actor {\n            #     __typename\n            #     ...custom_actor\n            #   }\n            #   requestedReviewer {\n            #     ...on User {\n            #       id\n            #       userName: name\n            #       userLogin: login\n            #     }\n            #     ...on Team {\n            #       id\n            #       teamName: name\n            #     }\n            #   }\n            #   createdAt\n            # }\n            # ... on ReviewRequestRemovedEvent {\n            #   id\n            #   actor {\n            #     __typename\n            #     ...custom_actor\n            #   }\n            #   requestedReviewer {\n            #     ...on User {\n            #       id\n            #       userName: name\n            #       userLogin: login\n            #     }\n            #     ...on Team {\n            #       id\n            #       teamName: name\n            #     }\n            #   }\n            #   createdAt\n            # }\n            ... on ReviewDismissedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              message\n              resourcePath\n              url\n              previousReviewState\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestReview($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequestReview {\n      comments(first: $first) {\n        nodes {\n          id\n          bodyText\n          viewerDidAuthor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery IssueTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              bodyText\n              createdAt\n              lastEditedAt\n              publishedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe\n              viewerSubscription\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              commit {\n                url\n                commitUrl\n              }\n              isCrossRepository\n              isDirectReference\n              createdAt\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              isCrossRepository\n              url\n              resourcePath\n              willCloseTarget\n              referencedAt\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              milestoneTitle\n              createdAt\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              currentTitle\n              previousTitle\n              createdAt\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              lockReason\n              createdAt\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n" ;
pub const OPERATION_NAME: &'static str = "PullRequestReview";
#[allow(dead_code)]
type Boolean = bool;
#[allow(dead_code)]
type Float = f64;
#[allow(dead_code)]
type Int = i64;
#[allow(dead_code)]
type ID = String;
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
pub struct PullRequestReviewNodeOnPullRequestReviewCommentsNodes {
    pub id: ID,
    #[serde(rename = "bodyText")]
    pub body_text: String,
    #[serde(rename = "viewerDidAuthor")]
    pub viewer_did_author: Boolean,
}
#[derive(Deserialize, Debug)]
pub struct PullRequestReviewNodeOnPullRequestReviewComments {
    pub nodes: Option<Vec<Option<PullRequestReviewNodeOnPullRequestReviewCommentsNodes>>>,
}
#[derive(Deserialize, Debug)]
pub struct PullRequestReviewNodeOnPullRequestReview {
    pub comments: PullRequestReviewNodeOnPullRequestReviewComments,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum PullRequestReviewNodeOn {
    PullRequestReview(PullRequestReviewNodeOnPullRequestReview),
    UnlockedEvent,
    User,
    CommitCommentThread,
    MarketplaceCategory,
    Organization,
    AddedToProjectEvent,
    PullRequestReviewThread,
    ReviewDismissedEvent,
    BaseRefChangedEvent,
    RenamedTitleEvent,
    Blob,
    CheckSuite,
    CommitComment,
    Language,
    StatusContext,
    DeploymentStatus,
    ReferencedEvent,
    Project,
    IssueComment,
    Issue,
    Commit,
    ReviewRequestRemovedEvent,
    DeployKey,
    RepositoryInvitation,
    HeadRefForcePushedEvent,
    PublicKey,
    Status,
    UnassignedEvent,
    Deployment,
    Topic,
    RepositoryTopic,
    ReviewRequestedEvent,
    PullRequestCommit,
    Label,
    License,
    GistComment,
    HeadRefDeletedEvent,
    ClosedEvent,
    OrganizationIdentityProvider,
    ReleaseAsset,
    PullRequestReviewComment,
    App,
    AssignedEvent,
    ProjectCard,
    Repository,
    Bot,
    PushAllowance,
    Tag,
    ReviewRequest,
    DemilestonedEvent,
    CommentDeletedEvent,
    PullRequest,
    CrossReferencedEvent,
    Gist,
    Reaction,
    ProjectColumn,
    Milestone,
    PullRequestCommitCommentThread,
    MilestonedEvent,
    Release,
    LabeledEvent,
    SubscribedEvent,
    UnsubscribedEvent,
    Ref,
    MovedColumnsInProjectEvent,
    CheckRun,
    ProtectedBranch,
    BaseRefForcePushedEvent,
    RemovedFromProjectEvent,
    ExternalIdentity,
    MergedEvent,
    DeployedEvent,
    ReopenedEvent,
    HeadRefRestoredEvent,
    DeploymentEnvironmentChangedEvent,
    Tree,
    UserContentEdit,
    ConvertedNoteToIssueEvent,
    MarketplaceListing,
    Push,
    ReviewDismissalAllowance,
    OrganizationInvitation,
    LockedEvent,
    UnlabeledEvent,
    Team,
    MentionedEvent,
}
#[derive(Deserialize, Debug)]
pub struct PullRequestReviewNode {
    #[serde(flatten)]
    pub on: PullRequestReviewNodeOn,
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
    pub node: Option<PullRequestReviewNode>,
    #[serde(flatten)]
    pub rate_limit_info: RateLimitInfo,
}

pub struct PullRequestReview;

impl<'de> graphql_client::GraphQLQuery<'de> for PullRequestReview {
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
