CREATE TABLE messages (
    id UUID PRIMARY KEY,
    -- App ID for wechat MP account
    app_id Text NOT NULL,
    -- message template ID
    template_id Text NOT NULL,
    receiver_id Text NOT NULL,
    title Text NOT NULL,
    body Text NOT NULL,
    url Text,
    created_time BIGINT NOT NULL,
    -- sender info
    ip Text NOT NULL,
    "UA" Text NOT NULL,
    -- wechat response info
    errcode INTEGER,
    msgid BIGINT NOT NULL
);
