table! {
    messages (id) {
        id -> Uuid,
        app_id -> Nullable<Varchar>,
        template_id -> Nullable<Varchar>,
        receiver_id -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        body -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        created_time -> Nullable<Int8>,
        ip -> Nullable<Varchar>,
        UA -> Nullable<Varchar>,
        errcode -> Nullable<Int4>,
        msgid -> Nullable<Int4>,
    }
}
