CREATE TABLE IF NOT EXISTS forms (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT
);

SELECT diesel_manage_updated_at('forms');
INSERT INTO forms (id, name, description) VALUES 
  (1, 'Form 1', 'Description 1'),
  (2, 'Form 2', 'Description 2'),
  (3, 'Form 3', 'Description 3'),
  (4, 'Form 4', 'Description 4')
  ;

-- update the sequence to the max id
SELECT setval('forms_id_seq', (SELECT MAX(id) FROM forms));

-- ALTER TABLE forms REPLICA IDENTITY FULL;
-- CREATE PUBLICATION forms_pub FOR TABLE forms;
