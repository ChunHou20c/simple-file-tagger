-- Your SQL goes here
CREATE TABLE file_records (
  id integer primary key autoincrement not null,
  filename text not null,
  created_at datetime,
  updated_at datetime
) 
