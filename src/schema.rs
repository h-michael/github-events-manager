table! {
    issue_event_conditions (id) {
        id -> Integer,
        repository_id -> Integer,
        start_condition -> Integer,
        stop_condition -> Integer,
        listen_status -> Integer,
    }
}

table! {
    issues (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        edited_at -> Nullable<Text>,
        closed_at -> Nullable<Text>,
        node_id -> Text,
        number -> Integer,
        repository_id -> Integer,
        state -> Text,
        title -> Nullable<Text>,
        body_text -> Text,
        closed -> Integer,
        last_issue_cursor -> Nullable<Text>,
    }
}

table! {
    pull_request_event_conditions (id) {
        id -> Integer,
        repository_id -> Integer,
        start_condition -> Integer,
        stop_condition -> Integer,
        listen_status -> Integer,
    }
}

table! {
    pull_requests (id) {
        id -> Integer,
        created_at -> Text,
        updated_at -> Text,
        edited_at -> Nullable<Text>,
        closed_at -> Nullable<Text>,
        merged_at -> Nullable<Text>,
        node_id -> Text,
        number -> Integer,
        repository_id -> Integer,
        state -> Text,
        title -> Text,
        body_text -> Text,
        closed -> Integer,
        merged -> Integer,
        last_pull_request_cursor -> Nullable<Text>,
    }
}

table! {
    repositories (id) {
        id -> Integer,
        node_id -> Text,
        owner -> Text,
        name -> Text,
        url -> Text,
        last_pr_cursor -> Nullable<Text>,
        last_issue_cursor -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    issue_event_conditions,
    issues,
    pull_request_event_conditions,
    pull_requests,
    repositories,
);
