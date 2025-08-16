CREATE TABLE transactions (
  id BLOB NOT NULL PRIMARY KEY,
  amount TEXT NOT NULL,
  recipient BLOB NOT NULL,
  sender BLOB NOT NULL,
  timestamp TEXT NOT NULL,
  FOREIGN KEY (recipient) REFERENCES users (id),
  FOREIGN KEY (sender) REFERENCES users (id)
) STRICT;
