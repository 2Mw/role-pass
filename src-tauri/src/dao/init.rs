use rusqlite::Connection;

use crate::util::crypto;

pub struct InitDao {}

impl InitDao {
    pub fn new() -> InitDao {
        InitDao {  }
    }

    /// dao for initial table `global_config`
    pub fn init_global_config(&self, db: &Connection) -> Result<(), rusqlite::Error> {
        db.execute(
            "CREATE TABLE global_config(
            ID INTEGER PRIMARY KEY   AUTOINCREMENT,
            app_password TEXT NOT NULL,
            create_time DATETIME NOT NULL,
            last_run DATETIME NOT NULL,
            run_count INTEGER NOT NULL)",
            (),
        )?;
        
    
        db.execute(
            "INSERT INTO global_config (
                app_password, create_time, last_run, run_count) 
                VALUES ('', datetime('now','localtime'), datetime('now','localtime'), 1);",
            (),
        )?;

        Ok(())
    }

    pub fn if_app_password_set(&self, db: &Connection) -> Result<bool, rusqlite::Error> {
        let row: Result<String, rusqlite::Error> = db.query_row(
            "SELECT app_password from global_config where ID = 1 ",
            (),
            |r| r.get(0),
        );

        let pass = row?;
        if pass.len() == 128 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }


    pub fn set_app_password(&self, db: &Connection, pass: String) -> Result<bool, rusqlite::Error> {
        let res = db.execute(
            "update global_config set app_password = ?1 where ID = 1;",
            [crypto::password_crypto(&pass)],
        )?;
        if res == 1 {
            Ok(true)
        }else {
            Ok(false)
        }
    }

    
    pub fn valid_app_password(&self, db: &Connection, pass: String) -> Result<bool, rusqlite::Error> {
        let row: Result<String, rusqlite::Error> = db.query_row(
            "SELECT app_password from global_config where ID = 1 ",
            (),
            |r| r.get(0),
        );

        let o_pass = row?;
        let c_pass = crypto::password_crypto(&pass);
        // println!("{}\n{}", o_pass, c_pass);
        if c_pass == o_pass {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}