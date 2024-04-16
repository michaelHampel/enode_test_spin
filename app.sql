CREATE TABLE IF NOT EXISTS enodeTokens (
   client TEXT PRIMARY KEY,
   token varchar NOT NULL,
   lifetime INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS userTokens (
   userid TEXT PRIMARY KEY,
   token varchar NOT NULL,
);

CREATE TABLE IF NOT EXISTS users (
   userid TEXT PRIMARY KEY,
   firstname varchar NOT NULL,
   lastname INTEGER NOT NULL
   pwd varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS user2enode (
   userid TEXT PRIMARY KEY,
   enodeid varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
   id INTEGER PRIMARY KEY AUTOINCREMENT, 
   description TEXT NOT NULL, 
   due TEXT NOT NULL
);