use rusqlite::Connection;

use crate::{models::user::User, util::crypto::password_crypto};

pub struct UserDao {}

impl UserDao {
    pub fn new() -> Self {
        UserDao {  }
    }

    pub fn create_user_tables(&self, db: &Connection) -> Result<bool, String> {
        let res = db.execute("CREATE table if not exists users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            create_time DATETIME NOT NULL,
            last_login DATETIME NOT NULL,
            login_times INTEGER NOT NULL
        )", []);
        if let Err(e) = res {
            return Err(format!("Create table users failed: {}", e));
        }

        let res = db.execute("CREATE table if not exists user_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid INTEGER NOT NULL,
            role TEXT NOT NULL,
            tip TEXT,
            constraint fk_user_id FOREIGN KEY (uid) references users(id)
        )", []);

        if let Err(e) = res {
            return Err(format!("Create table user_roles failed: {}", e));
        }

        let res = db.execute("create table if not exists user_password (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid INTEGER NOT NULL,
            rid INTEGER NOT NULL,
            account TEXT NOT NULL,
            password TEXT NOT NULL,
            create_time DATETIME NOT NULL,
            last_update DATETIME NOT NULL,
            update_count INTEGER NOT NULL,
            login_url TEXT,
            tip TEXT,
            constraint fk_user_id FOREIGN KEY (uid) references users(id),
            constraint fk_role_id FOREIGN KEY (rid) references user_roles(id)
        )", []);

        if let Err(e) = res {
            return Err(format!("Create table user_password failed: {}", e));
        }

        Ok(true)
    }

    pub fn query_users(&self, db: &Connection) -> Result<Vec<User>, String> {
        let st = db.prepare("SELECT * from users");
        match st {
            Ok(mut stmt) => {
                let rows = stmt.query_map([], |r| {
                    Ok(User {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        password: r.get(2)?,
                        create_time: r.get(3)?,
                        last_login: r.get(4)?,
                        login_times: r.get(5)?,
                    })
                });

                if let Ok(row) = rows {
                    let mut ret = Vec::new();
                    for p in row {
                        ret.push(p.unwrap());
                    }
                    return Ok(ret);
                }
                return Err(format!("Query failed where retrives rows."));
            },
            Err(e)=> {Err(format!("Query table users failed: {}", e))}
        }
        
    }

    pub fn login(&self, db: &Connection, id: usize, pass: String) -> Result<bool, String> {
        let mut st = db.prepare("SELECT password from users where id=?").unwrap();
        let o_pass:String = st.query_row([id], |r| {
            Ok(r.get(0).unwrap())
        }).unwrap();
        let c_pass = password_crypto(&pass);
        // println!("{}\n{}", o_pass, c_pass);
        if o_pass == c_pass { Ok(true) }
        else {Err("密码不正确".to_string())}
    }
}