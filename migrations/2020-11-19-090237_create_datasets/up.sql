CREATE TABLE datasets (
    id SERIAL PRIMARY KEY,
    seed BIGINT,
    gmm JSON NOT NULL
)
