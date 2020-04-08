table! {
    messages (id) {
        id -> Uuid,
        app_id -> Text,
        template_id -> Text,
        receiver_id -> Text,
        title -> Text,
        body -> Text,
        url -> Nullable<Text>,
        created_time -> Int8,
        ip -> Text,
        UA -> Text,
    }
}
