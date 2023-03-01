CREATE TABLE IF NOT EXISTS form_questions (
  id SERIAL PRIMARY KEY,
  form_id INTEGER NOT NULL,
  question_id INTEGER NOT NULL,
  CONSTRAINT fk_fquestions_form FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE,
  CONSTRAINT fk_fquestions_question FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('form_questions');

INSERT INTO form_questions (id, form_id, question_id) VALUES 
  (1, 1, 1),
  (2, 1, 2),
  (3, 1, 3),
  (4, 1, 4),
  (5, 1, 5),
  (6, 2, 1),
  (7, 2, 2),
  (8, 2, 3),
  (9, 2, 4),
  (10, 2, 5),
  (11, 3, 1),
  (12, 3, 2),
  (13, 3, 3),
  (14, 3, 4),
  (15, 3, 5),
  (16, 4, 1),
  (17, 4, 2),
  (18, 4, 3),
  (19, 4, 4),
  (20, 4, 5)
  ;

-- update the sequence to the max id
SELECT setval('form_questions_id_seq', (SELECT MAX(id) FROM form_questions));

-- ALTER TABLE form_questions REPLICA IDENTITY FULL;
-- CREATE PUBLICATION fquestions_pub FOR TABLE form_questions;
