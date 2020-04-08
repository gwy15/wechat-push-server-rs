CREATE TABLE messages (
    id UUID PRIMARY KEY,
    -- App ID for wechat MP account
    app_id VARCHAR(255),
    -- message template ID
    template_id VARCHAR(255),
    receiver_id VARCHAR(255),
    title VARCHAR(255),
    body VARCHAR(65535),
    url VARCHAR(4095),
    created_time BIGINT,
    -- sender info
    ip VARCHAR(127),
    "UA" VARCHAR(1023),
    -- wechat response info
    errcode INTEGER,
    msgid INTEGER
);
