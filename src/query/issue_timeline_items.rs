pub mod issue_timeline_item {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    use serde;
    pub const QUERY : & 'static str = "query IssueTimelineItems($issue_node_id: ID!, $first: Int = 100){\n  node(id: $issue_node_id) {\n    __typename\n    ... on Issue {\n      id\n      author {\n        __typename\n        ...custom_actor\n      }\n      editor {\n        __typename\n        ...custom_actor\n      }\n      title\n      body\n      bodyText\n      timeline(first: $first) {\n        edges {\n          node {\n            __typename\n            ... on Commit {\n              ...custom_commit\n            }\n            ... on IssueComment {\n              id\n            }\n            ... on CrossReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ClosedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReopenedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on Subscribable {\n              id\n              __typename\n              viewerCanSubscribe {\n                __typename\n                ...custom_actor\n              }\n              viewerSubscription {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnsubscribedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on ReferencedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on AssignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnassignedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on LabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnlabeledEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on MilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on DemilestonedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on RenamedTitleEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on LockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n            ... on UnlockedEvent {\n              id\n              actor {\n                __typename\n                ...custom_actor\n              }\n            }\n          }\n        }\n        pageInfo {\n          hasNextPage\n          endCursor\n        }\n      }\n    }\n  }\n}\n\nfragment custom_commit on Commit {\n  id\n  oid\n  messageBody\n  messageHeadline\n  commitUrl\n  committedDate\n  pushedDate\n}\n\nfragment custom_actor on Actor {\n  __typename\n  ... on User {\n    id\n    name\n    login\n  }\n  ... on Organization {\n    id\n    name\n    login\n  }\n  ... on Bot {\n    id\n    login\n  }\n}\n" ;
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        Issue,
        Team,
        Repository,
        PullRequest,
        Commit,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        User,
        Bot,
        Organization,
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
        Language,
        UnassignedEvent,
        CheckRun,
        UnsubscribedEvent,
        UnlockedEvent,
        DeployedEvent,
        MilestonedEvent,
        CheckSuite,
        ReleaseAsset,
        License,
        PullRequestCommit,
        Organization,
        Commit,
        ReviewRequest,
        Tree,
        Push,
        PullRequestCommitCommentThread,
        HeadRefDeletedEvent,
        RemovedFromProjectEvent,
        SubscribedEvent,
        HeadRefForcePushedEvent,
        ProtectedBranch,
        MentionedEvent,
        Milestone,
        LockedEvent,
        ReopenedEvent,
        BaseRefForcePushedEvent,
        DeployKey,
        ReferencedEvent,
        GistComment,
        LabeledEvent,
        Reaction,
        ConvertedNoteToIssueEvent,
        Bot,
        CommitComment,
        CommitCommentThread,
        AddedToProjectEvent,
        IssueComment,
        PublicKey,
        UserContentEdit,
        RepositoryInvitation,
        Status,
        Deployment,
        Team,
        Tag,
        Release,
        ReviewDismissedEvent,
        StatusContext,
        MovedColumnsInProjectEvent,
        Label,
        ReviewDismissalAllowance,
        MarketplaceListing,
        PullRequestReviewComment,
        ReviewRequestRemovedEvent,
        Topic,
        CrossReferencedEvent,
        PullRequest,
        DeploymentStatus,
        Gist,
        PushAllowance,
        MarketplaceCategory,
        UnlabeledEvent,
        Ref,
        HeadRefRestoredEvent,
        Blob,
        ProjectColumn,
        MergedEvent,
        DemilestonedEvent,
        BaseRefChangedEvent,
        OrganizationIdentityProvider,
        AssignedEvent,
        Repository,
        CommentDeletedEvent,
        ReviewRequestedEvent,
        OrganizationInvitation,
        Project,
        RepositoryTopic,
        App,
        PullRequestReviewThread,
        ClosedEvent,
        User,
        PullRequestReview,
        RenamedTitleEvent,
        ExternalIdentity,
        ProjectCard,
        DeploymentEnvironmentChangedEvent,
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
    }
}
impl<'de> ::graphql_client::GraphQLQuery<'de> for IssueTimelineItem {
    type Variables = issue_timeline_item::Variables;
    type ResponseData = issue_timeline_item::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        ::graphql_client::QueryBody {
            variables,
            query: issue_timeline_item::QUERY,
            operation_name: issue_timeline_item::OPERATION_NAME,
        }
    }
}
