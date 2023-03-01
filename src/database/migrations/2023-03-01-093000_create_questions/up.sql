CREATE TABLE IF NOT EXISTS questions (
    id SERIAL PRIMARY KEY,
    q_type VARCHAR(10) NOT NULL,
    question VARCHAR NOT NULL
);

ALTER TABLE questions REPLICA IDENTITY FULL;

INSERT INTO questions VALUES
  (1, 'range', '¿Bla?'),
  (2, 'range', '¿Bla bla?'),
  (3, 'range', '¿Bla bla bla?'),
  (4, 'range', '¿Bla bla bla bla?'),
  (5, 'input', '¿Why?') ON CONFLICT DO NOTHING
  ;

-- update the sequence to the max id
SELECT setval('questions_id_seq', (SELECT MAX(id) FROM questions));

CREATE PUBLICATION questions_pub FOR TABLE questions;
