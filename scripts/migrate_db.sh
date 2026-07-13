cp lbs.db lbs.db.bak
DATABASE_URL=sqlite://lbs.db sqlx migrate run
