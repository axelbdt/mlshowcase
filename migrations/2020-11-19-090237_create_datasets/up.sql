CREATE TABLE datasets (
    id SERIAL PRIMARY KEY,
    seed BIGINT NOT NULL,
    gmm JSONB[] NOT NULL
)
