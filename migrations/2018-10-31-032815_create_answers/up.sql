CREATE TABLE answers (
  id INT PRIMARY KEY AUTO_INCREMENT,
  question_id INT NOT NULL,
  title VARCHAR(255) NOT NULL,
  user_id INT NOT NULL,
  created TIMESTAMP NOT NULL
)
