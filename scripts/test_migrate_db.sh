scripts/backup_db.sh
sqlite3 db/lbs.db.bak ".backup db/lbs_test.db"
DATABASE_URL=sqlite://db/lbs_test.db lbs_migrate

