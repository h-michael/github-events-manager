#![allow(proc_macro_derive_resolution_fallback)]
table! {
    issue_event_conditions (id) {
        id -> Integer,
        repository_id -> Integer,
        start_condition -> Integer,
        stop_condition -> Integer,
    }
}

table! {
    pull_request_event_conditions (id) {
        id -> Integer,
        repository_id -> Integer,
        start_condition -> Integer,
        stop_condition -> Integer,
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
