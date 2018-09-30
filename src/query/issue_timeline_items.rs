pub mod issue_timeline_items {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    use serde;
    pub const QUERY : & 'static str = "query IssueTimelineItems($issue_node_id: ID!, $first: Int = 100){\n  node(id: $issue_node_id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe {\n                __typename\n                ...custom_actor\n              }\n              viewerSubscription {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n  ...RateLimitInfo\n}\n\nfragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n\nfragment RateLimitInfo on Query {\n    rateLimit {\n        cost\n        limit\n        remaining\n        resetAt\n    }\n}\n" ;
    pub const OPERATION_NAME: &'static str = "IssueTimelineItems";
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
    pub enum RustIssueTimelineItemsNodeOnIssueAuthorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueAuthor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueAuthorOn,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueEditorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueEditor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueEditorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCommit {
        #[serde(flatten)]
        pub custom_commit: custom_commit,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueComment {
        pub id: ID,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEvent {
        pub id: ID,
        pub actor:
            Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribableOn {
        Commit,
        PullRequest,
        Issue,
        Team,
        Repository,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribable {
        pub id: ID,
        #[serde(rename = "viewerCanSubscribe")]
        pub viewer_can_subscribe: Boolean,
        #[serde(rename = "viewerSubscription")]
        pub viewer_subscription: Option<SubscriptionState>,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribableOn,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEvent {
        pub id: ID,
        pub actor:
            Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnAssignedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnassignedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLabeledEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlabeledEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnMilestonedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEvent {
        pub id: ID,
        pub actor:
            Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnDemilestonedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEvent {
        pub id: ID,
        pub actor:
            Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnRenamedTitleEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnLockedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActorOn {
        Organization,
        User,
        Bot,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActor {
        #[serde(flatten)]
        pub custom_actor: custom_actor,
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActorOn,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEvent {
        pub id: ID,
        pub actor: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnlockedEventActor>,
    }
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOnIssueTimelineEdgesNode {
        Commit(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCommit),
        IssueComment(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnIssueComment),
        CrossReferencedEvent(
            RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnCrossReferencedEvent,
        ),
        ClosedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnClosedEvent),
        ReopenedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReopenedEvent),
        Subscribable(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnSubscribable),
        UnsubscribedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnUnsubscribedEvent),
        ReferencedEvent(RustIssueTimelineItemsNodeOnIssueTimelineEdgesNodeOnReferencedEvent),
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
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelineEdges {
        pub node: Option<RustIssueTimelineItemsNodeOnIssueTimelineEdgesNode>,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimelinePageInfo {
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNodeOnIssueTimeline {
        pub edges: Option<Vec<Option<RustIssueTimelineItemsNodeOnIssueTimelineEdges>>>,
        #[serde(rename = "pageInfo")]
        pub page_info: RustIssueTimelineItemsNodeOnIssueTimelinePageInfo,
    }
    #[derive(Deserialize, Degug)]
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
    #[derive(Deserialize, Degug)]
    #[serde(tag = "__typename")]
    pub enum RustIssueTimelineItemsNodeOn {
        Issue(RustIssueTimelineItemsNodeOnIssue),
        RepositoryTopic,
        MovedColumnsInProjectEvent,
        Tree,
        Gist,
        Commit,
        RepositoryInvitation,
        PublicKey,
        PullRequest,
        ConvertedNoteToIssueEvent,
        UserContentEdit,
        Bot,
        BaseRefForcePushedEvent,
        Blob,
        ExternalIdentity,
        DemilestonedEvent,
        Milestone,
        UnassignedEvent,
        ProjectColumn,
        OrganizationInvitation,
        Ref,
        PushAllowance,
        CrossReferencedEvent,
        ReviewRequestRemovedEvent,
        Tag,
        ReviewDismissedEvent,
        HeadRefDeletedEvent,
        ReleaseAsset,
        Team,
        UnsubscribedEvent,
        UnlockedEvent,
        AssignedEvent,
        PullRequestCommitCommentThread,
        MergedEvent,
        License,
        Repository,
        ReopenedEvent,
        ProjectCard,
        Label,
        Push,
        CommentDeletedEvent,
        PullRequestCommit,
        Language,
        MentionedEvent,
        DeploymentStatus,
        App,
        HeadRefRestoredEvent,
        PullRequestReviewThread,
        Reaction,
        CommitCommentThread,
        ReviewRequestedEvent,
        LockedEvent,
        ReviewDismissalAllowance,
        Status,
        SubscribedEvent,
        BaseRefChangedEvent,
        PullRequestReview,
        ClosedEvent,
        Topic,
        OrganizationIdentityProvider,
        Deployment,
        StatusContext,
        GistComment,
        CheckRun,
        Organization,
        CheckSuite,
        ReferencedEvent,
        MarketplaceListing,
        DeploymentEnvironmentChangedEvent,
        ProtectedBranch,
        CommitComment,
        RenamedTitleEvent,
        Project,
        DeployKey,
        IssueComment,
        RemovedFromProjectEvent,
        Release,
        DeployedEvent,
        MarketplaceCategory,
        HeadRefForcePushedEvent,
        UnlabeledEvent,
        AddedToProjectEvent,
        LabeledEvent,
        ReviewRequest,
        MilestonedEvent,
        User,
        PullRequestReviewComment,
    }
    #[derive(Deserialize, Degug)]
    pub struct RustIssueTimelineItemsNode {
        #[serde(flatten)]
        pub on: RustIssueTimelineItemsNodeOn,
    }
    #[derive(Serialize, Degug)]
    pub struct Variables {
        pub issue_node_id: ID,
        pub first: Option<Int>,
    }
    impl Variables {
        pub fn default_first() -> Option<Int> {
            Some(100i64)
        }
    }
    #[derive(Deserialize, Degug)]
    pub struct ResponseData {
        pub node: Option<RustIssueTimelineItemsNode>,
        #[serde(flatten)]
        pub rate_limit_info: RateLimitInfo,
    }
}
impl<'de> ::graphql_client::GraphQLQuery<'de> for IssueTimelineItems {
    type Variables = issue_timeline_items::Variables;
    type ResponseData = issue_timeline_items::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        ::graphql_client::QueryBody {
            variables,
            query: issue_timeline_items::QUERY,
            operation_name: issue_timeline_items::OPERATION_NAME,
        }
    }
}
