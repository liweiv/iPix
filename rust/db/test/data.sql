INSERT INTO user_provider (
        id,
        access_key,
        secret_key,
        host,
        prefix,
        naming_rule,
        provider_id
    )
VALUES(
        'afe9b0cf-9571-4cd0-97a8-f126dbe808c1',
        'ak',
        'sk',
        'https://ab.com',
        'test',
        '${xx}/${y}',
        'QINIU'
    );