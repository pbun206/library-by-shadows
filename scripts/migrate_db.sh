cp lbs.db lbs.db.bak
DATABASE_URL=sqlite://lb/lbs.db sqlx migrate run
