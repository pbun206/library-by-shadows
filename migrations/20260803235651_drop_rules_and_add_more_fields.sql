DROP TABLE listed_rules;

create virtual table vec_urls using vec0(
  url TEXT PRIMARY KEY,
  embeding float[768]
);


