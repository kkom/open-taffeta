table! {
    doors (id) {
        id -> Integer,
        name -> Text,
        address -> Text,
        buzzer_url -> Text,
        ring -> Bool,
        ring_ts -> Nullable<Integer>,
    }
}

table! {
    roles (id) {
        id -> Integer,
        name -> Text,
        user -> Nullable<Integer>,
    }
}

table! {
    userauth (id) {
        id -> Integer,
        user_id -> Integer,
        exp -> Timestamp,
        client_id -> Text,
        token -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        password -> Text,
        email -> Text,
        is_active -> Bool,
    }
}

joinable!(roles -> users (user));

allow_tables_to_appear_in_same_query!(doors, roles, userauth, users,);
