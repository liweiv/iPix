-- --------------------------------------
-- Table structure for oss provider
-- --------------------------------------
CREATE TABLE IF NOT EXISTS oss_provider (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    disable INT DEFAULT 0
);
-- --------------------------------------
-- Table structure for oss provider
-- --------------------------------------
CREATE TABLE IF NOT EXISTS user_provider (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    access_key TEXT NOT NULL,
    secret_key TEXT NOT NULL,
    host TEXT NOT NULL,
    prefix TEXT,
    naming_rule TEXT,
    provider_id TEXT NOT NULL
);
INSERT INTO oss_provider(id, name)
VALUES('QINIU', '七牛');
INSERT INTO oss_provider(id, name)
VALUES('ALIYUN', '阿里云OSS');
INSERT INTO oss_provider(id, name)
VALUES('QCLOUD', '腾讯云COS');