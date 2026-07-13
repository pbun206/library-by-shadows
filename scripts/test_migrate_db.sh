rm lbs_test.db
cp lbs.db lbs.db.bak
cp lbs.db.bak lbs_test.db
DATABASE_URL=sqlite://lbs_test.db sqlx migrate run
