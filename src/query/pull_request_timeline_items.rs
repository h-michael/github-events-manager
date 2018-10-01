#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use graphql_client;
use query::typedef::*;
use serde;

pub const QUERY : & 'static str = "fragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n\nquery LoginUser {\n  viewer {\n    login\n  }\n  ...RateLimitInfo\n}\n\nquery RateLimit {\n  ...RateLimitInfo\n}\n\nquery Repository($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequests($owner: String!, $name: String!, $first: Int = 100, $states: [PullRequestState!]) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    pullRequests(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n          merged\n          mergedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery Issues($owner: String!, $name: String!, $first: Int = 100, $states: [IssueState!]){\n  repository(owner: $owner, name: $name) {\n    id\n    url\n    issues(first: $first, states: $states) {\n      edges {\n        node {\n          id\n          number\n          state\n          title\n          bodyText\n          createdAt\n          updatedAt\n          lastEditedAt\n          closed\n          closedAt\n        }\n      }\n      pageInfo {\n        startCursor\n        endCursor\n        hasNextPage\n        hasPreviousPage\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery WatchingRepositories($first: Int!, $after: String){\n  viewer {\n    watching(after: $after, first: $first) {\n      nodes {\n        id,\n        nameWithOwner,\n        url\n      }\n      pageInfo {\n        hasNextPage,\n        endCursor\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequest {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on CommitCommentThread {\n              id\n              commit_of_comment: commit {\n                ...custom_commit\n              }\n            }\n            ... on PullRequestReview {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              commit_of_pr_review: commit {\n                ...custom_commit\n              }\n              state\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on PullRequestReviewThread {\n              id\n              comments(first: $first) {\n                edges {\n                  node {\n                    id\n                    bodyText\n                    createdAt\n                    updatedAt\n                    lastEditedAt\n                  }\n                }\n              }\n            }\n            ... on PullRequestReviewComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on IssueComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on ReviewRequestedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              requestedReviewer {\n                __typename\n                ...on User {\n                  id\n                  userName: name\n                  userLogin: login\n                }\n                ...on Team {\n                  id\n                  teamName: name\n                }\n              }\n              createdAt\n            }\n            ... on ReviewRequestRemovedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              requestedReviewer {\n                __typename\n                ...on User {\n                  id\n                  userName: name\n                  userLogin: login\n                }\n                ...on Team {\n                  id\n                  teamName: name\n                }\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery PullRequestReview($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on PullRequestReview {\n      comments(first: $first) {\n        nodes {\n          id\n          bodyText\n          viewerDidAuthor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nquery IssueTimelineItems($id: ID!, $first: Int = 100) {\n  node(id: $id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              bodyText\n              createdAt\n              lastEditedAt\n              publishedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              url\n              resourcePath\n              createdAt\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              createdAt\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              user {\n                name\n                login\n              }\n              createdAt\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n              label {\n                name\n                description\n                color\n              }\n              createdAt\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\n# # FullAttributes\n# query PullRequestTimelineItems($id: ID!, $first: Int = 100) {\n#   node(id: $id) {\n#     __typename\n#     ... on PullRequest {\n#       id\n#       author {\n#         __typename\n#         ...custom_actor\n#       }\n#       editor {\n#         __typename\n#         ...custom_actor\n#       }\n#       title\n#       body\n#       bodyText\n#       timeline(first: $first) {\n#         edges {\n#           node {\n#             __typename\n#             ... on Commit {\n#               ...custom_commit\n#             }\n#             ... on CommitCommentThread {\n#               id\n#               commit_of_comment: commit {\n#                 ...custom_commit\n#               }\n#             }\n#             ... on PullRequestReview {\n#               id\n#               author {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               commit_of_pr_review: commit {\n#                 ...custom_commit\n#               }\n#               state\n#               createdAt\n#               updatedAt\n#               lastEditedAt\n#             }\n#             ... on PullRequestReviewThread {\n#               id\n#               comments(first: $first) {\n#                 edges {\n#                   node {\n#                     id\n#                     bodyText\n#                     createdAt\n#                     updatedAt\n#                     lastEditedAt\n#                   }\n#                 }\n#               }\n#             }\n#             ... on PullRequestReviewComment {\n#               id\n#               bodyText\n#               createdAt\n#               updatedAt\n#               lastEditedAt\n#             }\n#             ... on IssueComment {\n#               id\n#               bodyText\n#               createdAt\n#               updatedAt\n#               lastEditedAt\n#             }\n#             ... on ClosedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on ReopenedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on Subscribable {\n#               id\n#               __typename\n#               viewerCanSubscribe\n#               viewerSubscription\n#             }\n#             ... on UnsubscribedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on MergedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               url\n#               resourcePath\n#               createdAt\n#             }\n#             ... on ReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               commit {\n#                 url\n#                 commitUrl\n#               }\n#               isCrossRepository\n#               isDirectReference\n#               createdAt\n#             }\n#             ... on CrossReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               isCrossRepository\n#               url\n#               resourcePath\n#               willCloseTarget\n#               referencedAt\n#               createdAt\n#             }\n#             ... on AssignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on UnassignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on LabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on UnlabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on MilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on DemilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on RenamedTitleEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               currentTitle\n#               previousTitle\n#               createdAt\n#             }\n#             ... on LockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               lockReason\n#               createdAt\n#             }\n#             ... on UnlockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on DeployedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#             }\n#             ... on DeploymentEnvironmentChangedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on HeadRefDeletedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               headRef {\n#                 id\n#                 name\n#                 prefix\n#               }\n#               headRefName\n#               createdAt\n#             }\n#             ... on HeadRefRestoredEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on HeadRefForcePushedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               afterCommit {\n#                 id\n#                 oid\n#               }\n#               beforeCommit {\n#                 id\n#                 oid\n#               }\n#               ref {\n#                 id\n#                 name\n#                 prefix\n#               }\n#               createdAt\n#             }\n#             ... on BaseRefForcePushedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               afterCommit {\n#                 id\n#                 oid\n#               }\n#               beforeCommit {\n#                 id\n#                 oid\n#               }\n#               ref {\n#                 id\n#                 name\n#                 prefix\n#               }\n#               createdAt\n#             }\n#             ... on ReviewRequestedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               requestedReviewer {\n#                 ...on User {\n#                   id\n#                   userName: name\n#                   userLogin: login\n#                 }\n#                 ...on Team {\n#                   id\n#                   teamName: name\n#                 }\n#               }\n#               createdAt\n#             }\n#             ... on ReviewRequestRemovedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               requestedReviewer {\n#                 ...on User {\n#                   id\n#                   userName: name\n#                   userLogin: login\n#                 }\n#                 ...on Team {\n#                   id\n#                   teamName: name\n#                 }\n#               }\n#               createdAt\n#             }\n#             ... on ReviewDismissedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               message\n#               resourcePath\n#               url\n#               previousReviewState\n#               createdAt\n#             }\n#           }\n#         }\n#         pageInfo {\n#           hasNextPage\n#           endCursor\n#         }\n#       }\n#     }\n#   }\n#   ...RateLimitInfo\n# }\n\n# # FullAttributes\n# query IssueTimelineItems($id: ID!, $first: Int = 100) {\n#   node(id: $id) {\n#     __typename\n#     ... on Issue {\n#       id\n#       author {\n#         __typename\n#         ...custom_actor\n#       }\n#       editor {\n#         __typename\n#         ...custom_actor\n#       }\n#       title\n#       body\n#       bodyText\n#       timeline(first: $first) {\n#         edges {\n#           node {\n#             __typename\n#             ... on Commit {\n#               ...custom_commit\n#             }\n#             ... on IssueComment {\n#               id\n#               author {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               bodyText\n#               createdAt\n#               lastEditedAt\n#               publishedAt\n#             }\n#             ... on ClosedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               url\n#               resourcePath\n#               createdAt\n#             }\n#             ... on ReopenedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on Subscribable {\n#               id\n#               __typename\n#               viewerCanSubscribe\n#               viewerSubscription\n#             }\n#             ... on UnsubscribedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#             ... on ReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               commit {\n#                 url\n#                 commitUrl\n#               }\n#               isCrossRepository\n#               isDirectReference\n#               createdAt\n#             }\n#             ... on CrossReferencedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               isCrossRepository\n#               url\n#               resourcePath\n#               willCloseTarget\n#               referencedAt\n#               createdAt\n#             }\n#             ... on AssignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on UnassignedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               user {\n#                 name\n#                 login\n#               }\n#               createdAt\n#             }\n#             ... on LabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on UnlabeledEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               label {\n#                 name\n#                 description\n#                 color\n#               }\n#               createdAt\n#             }\n#             ... on MilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on DemilestonedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               milestoneTitle\n#               createdAt\n#             }\n#             ... on RenamedTitleEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               currentTitle\n#               previousTitle\n#               createdAt\n#             }\n#             ... on LockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               lockReason\n#               createdAt\n#             }\n#             ... on UnlockedEvent {\n#               id\n#               actor {\n#                 __typename\n#                 ...custom_actor\n#               }\n#               createdAt\n#             }\n#           }\n#         }\n#         pageInfo {\n#           hasNextPage\n#           endCursor\n#         }\n#       }\n#     }\n#   }\n#   ...RateLimitInfo\n# }\n" ;
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
    User,
    Bot,
    Organization,
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
    User,
    Bot,
    Organization,
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
    User,
    Bot,
    Organization,
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
    User,
    Bot,
    Organization,
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
    User,
    Bot,
    Organization,
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
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActorOn {
    User,
    Bot,
    Organization,
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
    User,
    Bot,
    Organization,
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
    User,
    Bot,
    Organization,
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
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActorOn
{
    User,
    Bot,
    Organization,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventRequestedReviewerOnUser
{
    pub id: ID,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    #[serde(rename = "userLogin")]
    pub user_login: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventRequestedReviewerOnTeam
{
    pub id: ID,
    #[serde(rename = "teamName")]
    pub team_name: String,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventRequestedReviewer
{
     User ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventRequestedReviewerOnUser ) , Team ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventRequestedReviewerOnTeam ) }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActor > , # [ serde ( rename = "requestedReviewer" ) ] pub requested_reviewer : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventRequestedReviewer > , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActorOn
{
    User,
    Bot,
    Organization,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActorOn , }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventRequestedReviewerOnUser
{
    pub id: ID,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    #[serde(rename = "userLogin")]
    pub user_login: String,
}
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventRequestedReviewerOnTeam
{
    pub id: ID,
    #[serde(rename = "teamName")]
    pub team_name: String,
}
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventRequestedReviewer
{
     User ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventRequestedReviewerOnUser ) , Team ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventRequestedReviewerOnTeam ) }
#[derive(Deserialize, Debug)]
pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActor > , # [ serde ( rename = "requestedReviewer" ) ] pub requested_reviewer : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventRequestedReviewer > , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , }
#[derive(Deserialize, Debug)]
#[serde(tag = "__typename")]
pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNode {
     Commit ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommit ) , CommitCommentThread ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThread ) , PullRequestReview ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReview ) , PullRequestReviewThread ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThread ) , PullRequestReviewComment ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewComment ) , IssueComment ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnIssueComment ) , ClosedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEvent ) , ReopenedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEvent ) , AssignedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEvent ) , UnassignedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEvent ) , LabeledEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEvent ) , ReviewRequestedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEvent ) , ReviewRequestRemovedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEvent ) , BaseRefForcePushedEvent , CrossReferencedEvent , DemilestonedEvent , DeployedEvent , DeploymentEnvironmentChangedEvent , HeadRefDeletedEvent , HeadRefForcePushedEvent , HeadRefRestoredEvent , LockedEvent , MergedEvent , MilestonedEvent , ReferencedEvent , RenamedTitleEvent , ReviewDismissedEvent , SubscribedEvent , UnlabeledEvent , UnlockedEvent , UnsubscribedEvent }
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
    ReleaseAsset,
    Tag,
    User,
    BaseRefChangedEvent,
    AssignedEvent,
    LockedEvent,
    CommentDeletedEvent,
    RemovedFromProjectEvent,
    Project,
    Team,
    UnlockedEvent,
    PullRequestReviewThread,
    Label,
    ClosedEvent,
    DeployKey,
    MarketplaceCategory,
    MarketplaceListing,
    ReviewDismissalAllowance,
    BaseRefForcePushedEvent,
    PullRequestCommit,
    HeadRefDeletedEvent,
    Deployment,
    ReviewRequestRemovedEvent,
    Topic,
    CheckSuite,
    Gist,
    ReviewDismissedEvent,
    License,
    Bot,
    ReviewRequestedEvent,
    DemilestonedEvent,
    CommitComment,
    CheckRun,
    IssueComment,
    HeadRefForcePushedEvent,
    PullRequestReview,
    ReopenedEvent,
    SubscribedEvent,
    GistComment,
    MovedColumnsInProjectEvent,
    UserContentEdit,
    ProtectedBranch,
    OrganizationIdentityProvider,
    AddedToProjectEvent,
    Reaction,
    UnlabeledEvent,
    Status,
    RepositoryInvitation,
    DeployedEvent,
    PushAllowance,
    RenamedTitleEvent,
    StatusContext,
    PublicKey,
    LabeledEvent,
    MilestonedEvent,
    Release,
    DeploymentStatus,
    ReferencedEvent,
    RepositoryTopic,
    Blob,
    Language,
    HeadRefRestoredEvent,
    PullRequestReviewComment,
    App,
    MergedEvent,
    ReviewRequest,
    Issue,
    ExternalIdentity,
    ConvertedNoteToIssueEvent,
    Milestone,
    Commit,
    Organization,
    MentionedEvent,
    CrossReferencedEvent,
    Push,
    PullRequestCommitCommentThread,
    UnassignedEvent,
    UnsubscribedEvent,
    Tree,
    CommitCommentThread,
    OrganizationInvitation,
    Repository,
    ProjectCard,
    ProjectColumn,
    Ref,
    DeploymentEnvironmentChangedEvent,
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
