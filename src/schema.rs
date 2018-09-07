table! {
    issue_event_conditions (id) {
        id -> Integer,
        repository_id -> Integer,
        open -> Integer,
        closed -> Integer,
    }
}

table! {
    pull_request_event_conditions (id) {
        id -> Integer,
        repository_id -> Integer,
        open -> Integer,
        closed -> Integer,
        merged -> Integer,
    }
}

table! {
    repositories (id) {
        id -> Integer,
        repository_id -> Text,
        owner -> Text,
        name -> Text,
        url -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    issue_event_conditions,
    pull_request_event_conditions,
    repositories,
);
