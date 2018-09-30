pub mod pr_timeline_items {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    use serde;
    pub const QUERY : & 'static str = "query PullRequestTimelineItems($pr_node_id: ID!, $first: Int = 100) {\n  node(id: $pr_node_id) {\n    __typename\n    ... on PullRequest {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on CommitCommentThread {\n              id\n              commit_of_comment: commit {\n                ...custom_commit\n              }\n            }\n            ... on PullRequestReview {\n              id\n              author {\n                __typename\n                ...custom_actor\n              }\n              commit_of_pr_review: commit {\n                ...custom_commit\n              }\n              state\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on PullRequestReviewThread {\n              id\n              comments(first: $first) {\n                edges {\n                  node {\n                    id\n                    bodyText\n                    createdAt\n                    updatedAt\n                    lastEditedAt\n                  }\n                }\n              }\n            }\n            ... on PullRequestReviewComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on IssueComment {\n              id\n              bodyText\n              createdAt\n              updatedAt\n              lastEditedAt\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe {\n                __typename\n                custom_actor\n              }\n              viewerSubscription {\n                __typename\n                custom_actor\n              }\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on MergedEvent {\n              id\n              createdAt\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DeployedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DeploymentEnvironmentChangedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on HeadRefDeletedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on HeadRefRestoredEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on HeadRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on BaseRefForcePushedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReviewRequestedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReviewRequestRemovedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReviewDismissedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nfragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n" ;
    pub const OPERATION_NAME: &'static str = "PullRequestTimelineItems";
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type DateTime = super::DateTime;
    type GitObjectID = super::GitObjectID;
    type URI = super::URI;
    #[derive(Degug)]
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
    #[derive(Degug)]
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
    #[derive(Deserialize, Degug)]
    pub struct RateLimitInfo {
        #[serde(rename = "rateLimit")]
        pub rate_limit: Option<RateLimitInfoRateLimit>,
    }
    #[derive(Deserialize, Degug)]
    pub struct RateLimitInfoRateLimit {
        pub cost: Int,
        pub limit: Int,
        pub remaining: Int,
        #[serde(rename = "resetAt")]
        pub reset_at: DateTime,
    }
    #[derive(Deserialize, Degug)]
    pub struct custom_actor {}
    #[derive(Deserialize, Degug)]
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
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestAuthorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestAuthor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestAuthorOn,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestEditorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestEditor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestEditorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommit {
        #[serde(flatten)]
        pub custom_commit: custom_commit,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThreadCommitOfComment
    {
        #[serde(flatten)]
        pub custom_commit: custom_commit,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThread { pub id : ID , pub commit_of_comment : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThreadCommitOfComment , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewCommitOfPrReview
    {
        #[serde(flatten)]
        pub custom_commit: custom_commit,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReview { pub id : ID , pub author : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewAuthor > , pub commit_of_pr_review : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewCommitOfPrReview > , pub state : PullRequestReviewState , # [ serde ( rename = "createdAt" ) ] pub created_at : DateTime , # [ serde ( rename = "updatedAt" ) ] pub updated_at : DateTime , # [ serde ( rename = "lastEditedAt" ) ] pub last_edited_at : Option < DateTime > , }
    #[derive(Deserialize, Degug)]
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
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdges { pub node : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdgesNode > , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadComments { pub edges : Option < Vec < Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadCommentsEdges > > > , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThread { pub id : ID , pub comments : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThreadComments , }
    #[derive(Deserialize, Degug)]
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
    #[derive(Deserialize, Degug)]
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
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribableOn {
        Issue,
        Team,
        PullRequest,
        Repository,
        Commit,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribable {
        pub id: ID,
        #[serde(rename = "viewerCanSubscribe")]
        pub viewer_can_subscribe: Boolean,
        #[serde(rename = "viewerSubscription")]
        pub viewer_subscription: Option<SubscriptionState>,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribableOn,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEvent {
        pub id: ID,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActorOn {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on:
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEvent {
        pub id: ID,
        pub actor: Option<
            RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEventActor,
        >,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActorOn
    {
        User,
        Bot,
        Organization,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActor { # [ serde ( flatten ) ] pub custom_actor : custom_actor , # [ serde ( flatten ) ] pub on : RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActorOn , }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEvent { pub id : ID , pub actor : Option < RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEventActor > , }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNode {
         Commit ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommit ) , CommitCommentThread ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCommitCommentThread ) , PullRequestReview ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReview ) , PullRequestReviewThread ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewThread ) , PullRequestReviewComment ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnPullRequestReviewComment ) , IssueComment ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnIssueComment ) , ClosedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnClosedEvent ) , ReopenedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReopenedEvent ) , Subscribable ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnSubscribable ) , UnsubscribedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnsubscribedEvent ) , MergedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMergedEvent ) , ReferencedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReferencedEvent ) , CrossReferencedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnCrossReferencedEvent ) , AssignedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnAssignedEvent ) , UnassignedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnassignedEvent ) , LabeledEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLabeledEvent ) , UnlabeledEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlabeledEvent ) , MilestonedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnMilestonedEvent ) , DemilestonedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDemilestonedEvent ) , RenamedTitleEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnRenamedTitleEvent ) , LockedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnLockedEvent ) , UnlockedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnUnlockedEvent ) , DeployedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeployedEvent ) , DeploymentEnvironmentChangedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnDeploymentEnvironmentChangedEvent ) , HeadRefDeletedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefDeletedEvent ) , HeadRefRestoredEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefRestoredEvent ) , HeadRefForcePushedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnHeadRefForcePushedEvent ) , BaseRefForcePushedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnBaseRefForcePushedEvent ) , ReviewRequestedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestedEvent ) , ReviewRequestRemovedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewRequestRemovedEvent ) , ReviewDismissedEvent ( RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNodeOnReviewDismissedEvent ) , SubscribedEvent }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdges {
        pub node: Option<RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdgesNode>,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimelinePageInfo {
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNodeOnPullRequestTimeline {
        pub edges: Option<Vec<Option<RustPullRequestTimelineItemsNodeOnPullRequestTimelineEdges>>>,
        #[serde(rename = "pageInfo")]
        pub page_info: RustPullRequestTimelineItemsNodeOnPullRequestTimelinePageInfo,
    }
    #[derive(Deserialize, Degug)]
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
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustPullRequestTimelineItemsNodeOn {
        PullRequest(RustPullRequestTimelineItemsNodeOnPullRequest),
        Bot,
        ConvertedNoteToIssueEvent,
        Language,
        Milestone,
        StatusContext,
        Ref,
        CheckRun,
        DeploymentEnvironmentChangedEvent,
        LabeledEvent,
        HeadRefRestoredEvent,
        Project,
        User,
        ProjectCard,
        GistComment,
        CommentDeletedEvent,
        AddedToProjectEvent,
        MarketplaceCategory,
        DeployedEvent,
        DeploymentStatus,
        OrganizationIdentityProvider,
        ExternalIdentity,
        PullRequestCommit,
        ClosedEvent,
        OrganizationInvitation,
        ReferencedEvent,
        Label,
        Organization,
        Gist,
        Reaction,
        Deployment,
        RemovedFromProjectEvent,
        Status,
        Topic,
        ReleaseAsset,
        Issue,
        HeadRefDeletedEvent,
        DemilestonedEvent,
        PublicKey,
        UnassignedEvent,
        PullRequestReview,
        Release,
        UserContentEdit,
        RenamedTitleEvent,
        MilestonedEvent,
        ReviewRequestedEvent,
        PullRequestCommitCommentThread,
        IssueComment,
        MentionedEvent,
        MergedEvent,
        BaseRefChangedEvent,
        ReopenedEvent,
        LockedEvent,
        MarketplaceListing,
        ProjectColumn,
        CrossReferencedEvent,
        ProtectedBranch,
        Repository,
        Tree,
        ReviewDismissalAllowance,
        Commit,
        BaseRefForcePushedEvent,
        Tag,
        Blob,
        App,
        AssignedEvent,
        SubscribedEvent,
        CheckSuite,
        RepositoryInvitation,
        Team,
        PullRequestReviewComment,
        ReviewRequestRemovedEvent,
        UnsubscribedEvent,
        MovedColumnsInProjectEvent,
        PullRequestReviewThread,
        UnlockedEvent,
        ReviewDismissedEvent,
        Push,
        ReviewRequest,
        HeadRefForcePushedEvent,
        UnlabeledEvent,
        CommitComment,
        DeployKey,
        PushAllowance,
        License,
        RepositoryTopic,
        CommitCommentThread,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustPullRequestTimelineItemsNode {
        #[serde(flatten)]
        pub on: RustPullRequestTimelineItemsNodeOn,
    }
    #[derive(Serialize, Degug)]
    pub struct Variables {
        pub pr_node_id: ID,
        pub first: Option<Int>,
    }
    impl Variables {
        pub fn default_first() -> Option<Int> {
            Some(100i64)
        }
    }
    #[derive(Deserialize, Degug)]
    pub struct ResponseData {
        pub node: Option<RustPullRequestTimelineItemsNode>,
        #[serde(flatten)]
        pub rate_limit_info: RateLimitInfo,
    }
}
impl<'de> ::graphql_client::GraphQLQuery<'de> for PrTimelineItems {
    type Variables = pr_timeline_items::Variables;
    type ResponseData = pr_timeline_items::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        ::graphql_client::QueryBody {
            variables,
            query: pr_timeline_items::QUERY,
            operation_name: pr_timeline_items::OPERATION_NAME,
        }
    }
}
