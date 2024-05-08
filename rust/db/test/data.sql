INSERT INTO user_provider (
        access_key,
        secret_key,
        host,
        prefix,
        naming_rule,
        provider_id
    )
VALUES(
        'ak',
        'sk',
        'https://ab.com',
        'test',
        '${xx}/${y}',
        'QINIU'
    );