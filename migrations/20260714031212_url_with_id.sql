INSERT INTO url_fts5_idx(url_fts5_idx) VALUES('rebuild');

-- https://sqlite.org/fts5.html#external_content_tables
-- Triggers to keep the FTS index up to date.
CREATE TRIGGER urls_ai AFTER INSERT ON urls BEGIN
  INSERT INTO url_fts5_idx(rowid, url, title, description, content) VALUES (new.rowid, new.url, new.title, new.description, new.content);
END;
CREATE TRIGGER urls_ad AFTER DELETE ON urls BEGIN
  INSERT INTO url_fts5_idx(url_fts5_idx, rowid, url, title, description, content) VALUES ('delete', old.rowid, old.url, old.title, old.description, old.content);
END;
CREATE TRIGGER urls_au AFTER UPDATE ON urls BEGIN
  INSERT INTO url_fts5_idx(url_fts5_idx, rowid, url, title, description, content) VALUES ('delete', old.rowid, old.url, old.title, old.description, old.content);
  INSERT INTO url_fts5_idx(rowid, url, title, description, content) VALUES (new.rowid, new.url, new.title, new.description, new.content);

END;
