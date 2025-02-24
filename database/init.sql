CREATE TABLE link
(
    id                 serial  PRIMARY KEY,
    url                text  NOT NULL,
    alias              text NOT NULL ,
    development_fields jsonb NOT NULL DEFAULT '{}'
);
