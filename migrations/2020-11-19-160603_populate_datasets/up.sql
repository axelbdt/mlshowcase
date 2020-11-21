-- Your SQL goes here
INSERT INTO datasets (seed, gmm)
VALUES (
        42,
        ARRAY['{
            "cov": [3.0, 2.0, 2.0, 3.0],
            "mean": [-3.0, 3.0]
        }',
        '{
            "cov": [2.0, 1.0, 1.0, 2.0],
            "mean": [5.0, 2.0]
        }',
        '{
            "cov": [2.0, 0.0, 0.0, 2.0],
            "mean": [0.0, -3.0]
        }']::jsonb[]
    ),
    -- Second dataset
    (
        12,
        ARRAY['{
            "cov": [2.0, 1.0, 1.0, 2.0],
            "mean": [-3.0, -3.0]
        }',
        '{
            "cov": [3.0, 0.0, 0.0, 2.0],
            "mean": [2.0, 2.0]
        }']::jsonb[]
    )
