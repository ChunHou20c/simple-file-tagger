CREATE TABLE tag_records (
  id integer primary key autoincrement not null,
  file_id integer not null references file_records(id),
  tag_id integer not null references tags(id)
)
