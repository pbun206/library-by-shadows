Library by Shadows

# Modules

migrations -> sqlx tables 

handlers -> http handlers

models -> structs for database

dto -> data structure objects

services -> code logic including database edits

# Set-Up and Basics

if you want to actually use the server for adding urls, lmk and i can release the code for the browser extesnion. it's in firefox, but i'll vibe code a chrome version. however to make it work in firefox, you want to do some settings

Run `sqlx database create` and `sqlx migrate run` to initialize database.

`just build` -> builds project

`just run` -> builds project and runs 

`scripts/backup_db.sh` -> backup the current database

`scripts/migrate_db.sh` -> backups, migrate the current database

You can use cargo build or run, but you need just for the tailwind building.

During initialization, the model will be downloaded.
 
Please use rust analyzer, clippy, and your rust toolchain.

# Documentation

I'm honestly very lenient. However, I would like any comments for Option<_> or any unintuitive functions.

# Testing

Make sure the model is downloaded, or all the tests will fail.

`cargo check` -> check for compile errors in rust side.

`cargo test` -> test software works

`scripts/test_migrate_db.sh` -> backups, test migrate a backup version of the current database 

Write tests if possible 

# AI Use

You can use AI in any way you want, though if you wrote massive changes with solely AI, to be honest, I might not read it. Also, you must make an AI statement if you made AI or not. Otherwise, I would be least likely to look at it. 
