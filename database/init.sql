CREATE TABLE link
(
    id                 text PRIMARY KEY,
    url                text  NOT NULL,
    development_fields jsonb NOT NULL DEFAULT '{}'
);

CREATE TABLE ALIAS
(
    aliasId text PRIMARY KEY,
    url     text not NULL,
    expiration_time int not null ,
);