scripts/backup_db.sh
DATABASE_URL=sqlite://db/lbs.db sqlx migrate run
