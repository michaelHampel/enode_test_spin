CREATE TABLE IF NOT EXISTS enodeTokens (
   client TEXT PRIMARY KEY,
   token varchar NOT NULL,
   lifetime INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS userTokens (
   email TEXT PRIMARY KEY,
   token varchar NOT NULL,
);

CREATE TABLE IF NOT EXISTS users (
   email TEXT PRIMARY KEY,
   firstname varchar NOT NULL,
   lastname INTEGER NOT NULL
   pwd varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS user2enode (
   email TEXT PRIMARY KEY,
   enodeid varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
   id INTEGER PRIMARY KEY AUTOINCREMENT, 
   description TEXT NOT NULL, 
   due TEXT NOT NULL
);