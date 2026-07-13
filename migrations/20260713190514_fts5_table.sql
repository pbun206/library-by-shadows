ALTER TABLE url RENAME TO urls;
CREATE VIRTUAL TABLE url_fts5_idx USING fts5(
    title, description, content, 
    content='urls', content_rowid='rowid'
);
INSERT INTO url_fts5_idx(url_fts5_idx) VALUES('rebuild');
