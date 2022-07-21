use rusqlite::Connection;

use crate::{
    models::user::{User, UserPasswords},
    util::crypto::{decrypt, encrypt, password_crypto},
};

pub struct UserDao {}

impl UserDao {
    pub fn new() -> Self {
        UserDao {}
    }

    pub fn create_user_tables(&self, db: &Connection) -> Result<bool, String> {
        let res = db.execute(
            "CREATE table if not exists users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            create_time DATETIME NOT NULL,
            last_login DATETIME NOT NULL,
            login_times INTEGER NOT NULL
        )",
            [],
        );
        if let Err(e) = res {
            return Err(format!("Create table users failed: {}", e));
        }

        // let res = db.execute(
        //     "CREATE table if not exists user_roles (
        //     id INTEGER PRIMARY KEY AUTOINCREMENT,
        //     uid INTEGER NOT NULL,
        //     role TEXT NOT NULL,
        //     tip TEXT,
        //     constraint fk_user_id FOREIGN KEY (uid) references users(id)
        // )",
        //     [],
        // );

        // if let Err(e) = res {
        //     return Err(format!("Create table user_roles failed: {}", e));
        // }

        let res = db.execute(
            "create table if not exists user_password (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uid INTEGER NOT NULL,
            role TEXT NOT NULL,
            account TEXT NOT NULL,
            password TEXT NOT NULL,
            create_time DATETIME NOT NULL,
            last_update DATETIME NOT NULL,
            update_count INTEGER NOT NULL,
            login_url TEXT,
            tip TEXT,
            constraint fk_user_id FOREIGN KEY (uid) references users(id)
        )",
            [],
        );

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
            }
            Err(e) => Err(format!("Query table users failed: {}", e)),
        }
    }

    pub fn login(&self, db: &Connection, id: usize, pass: String) -> Result<bool, String> {
        let mut st = db.prepare("SELECT password from users where id=?").unwrap();
        let o_pass: String = st.query_row([id], |r| Ok(r.get(0).unwrap())).unwrap();
        let c_pass = password_crypto(&pass);
        // println!("{}\n{}", o_pass, c_pass);
        if o_pass == c_pass {
            db.execute(
                "update users set 
            login_times=login_times+1, last_login=strftime('%s','now')
            where id = ?",
                [id],
            )
            .unwrap();
            Ok(true)
        } else {
            Err("密码不正确".to_string())
        }
    }

    /// 创建用户
    pub fn create_user(
        &self,
        db: &Connection,
        name: &String,
        pass: &String,
    ) -> Result<bool, String> {
        let mut st = db
            .prepare("SELECT count(*) from users where name = ?")
            .unwrap();
        let cnt: usize = st.query_row([name], |r| Ok(r.get(0).unwrap())).unwrap();
        if cnt > 0 {
            // 有重复的用户名
            return Err("用户名已存在".to_string());
        }
        let c_pass = password_crypto(&pass);
        let res = db.execute(
            "INSERT INTO users values (null, ?, ?, strftime('%s','now'), strftime('%s','now'), 0)",
            [name, &c_pass],
        );

        match res {
            Ok(rows) => {
                if rows > 0 {
                    Ok(true)
                } else {
                    Err("创建用户失败，未知错误".to_string())
                }
            }
            Err(e) => Err(format!("Insert user failed: {}", e)),
        }
    }

    pub fn get_roles_by_id(
        &self,
        db: &Connection,
        uid: usize,
    ) -> Result<Vec<UserPasswords>, String> {
        let mut st = db
            .prepare("SELECT DISTINCT role FROM user_password where uid = ?")
            .unwrap();
        let rows = st.query_map([uid], |r| {
            Ok(UserPasswords {
                id: 0,
                uid,
                role: r.get(0)?,
                account: "".to_string(),
                password: "".to_string(),
                create_time: 0,
                last_update: 0,
                update_count: 0,
                login_url: "".to_string(),
                tip: "".to_string(),
            })
        });
        if let Ok(row) = rows {
            let mut ret = Vec::new();
            for p in row {
                ret.push(p.unwrap());
            }
            return Ok(ret);
        }
        Err(format!("Query table user_password failed"))
    }

    pub fn insert_account(
        &self,
        db: &Connection,
        uid: usize,
        role: String,
        account: String,
        pass: String,
        login_url: String,
        tip: String,
        upass: String,
    ) -> Result<bool, String> {
        // 1. 使用用户密码 upass密钥 对称加密字段 `account, pass, login_url, tip`
        let account = encrypt(&upass, &account);
        let pass = encrypt(&upass, &pass);
        let login_url = encrypt(&upass, &login_url);
        let tip = encrypt(&upass, &tip);
        // 2. 插入数据
        let res = db.execute(
            "INSERT INTO user_password values
        (null, ?, ?, ?, ? ,strftime('%s','now') ,strftime('%s','now'), 0, ?, ?)",
            rusqlite::params![uid, role, account, pass, login_url, tip],
        );
        match res {
            Ok(rows) => {
                if rows == 1 {
                    return Ok(true);
                }
            }
            Err(e) => {
                return Err(format!("Insert an acount info failed: {}", e));
            }
        }
        Err("Unknown error".to_string())
    }

    pub fn get_accounts_by_id(
        &self,
        db: &Connection,
        uid: usize,
        key: &String,
    ) -> Result<Vec<UserPasswords>, String> {
        let mut st = db
            .prepare("SELECT * FROM user_password where uid = ?")
            .unwrap();
        let rows = st.query_map([uid], |r| {
            let account: String = r.get(3)?;
            let account = decrypt(key, &account);
            let password: String = r.get(4)?;
            let password = decrypt(key, &password);
            let login_url: String = r.get(8)?;
            let login_url = decrypt(key, &login_url);
            let tip: String = r.get(9)?;
            let tip = decrypt(key, &tip);
            Ok(UserPasswords {
                id: r.get(0)?,
                uid: uid,
                role: r.get(2)?,
                account,
                password,
                create_time: r.get(5)?,
                last_update: r.get(6)?,
                update_count: r.get(7)?,
                login_url,
                tip,
            })
        });
        if let Ok(row) = rows {
            let mut ret = Vec::new();
            for p in row {
                ret.push(p.unwrap());
            }
            return Ok(ret);
        }
        Err(format!("Query table user_password failed"))
    }

    pub fn del_accounts_by_id(&self, db: &Connection, id: usize) -> Result<bool, String> {
        let res = db.execute("DELETE FROM user_password where id = ?", [id]);
        match res {
            Ok(rows) => {
                if rows == 1 {
                    Ok(true)
                } else {
                    Err("删除遇到未知错误".to_string())
                }
            }
            Err(e) => Err(format!("Delete error: {}", e)),
        }
    }

    pub fn update_accounts_by_id(
        &self,
        db: &Connection,
        id: usize,
        account: String,
        pass: String,
        login_url: String,
        tip: String,
        upass: String,
    ) -> Result<bool, String> {
        let account = encrypt(&upass, &account);
        let pass = encrypt(&upass, &pass);
        let login_url = encrypt(&upass, &login_url);
        let tip = encrypt(&upass, &tip);
        let res = db.execute(
            "update user_password set 
        account = ?, password = ?, login_url=?, tip = ?, last_update = strftime('%s','now'), update_count = update_count + 1
        where id = ?",
            rusqlite::params![account, pass, login_url, tip, id],
        );
        match res {
            Ok(rows) => {
                if rows == 1 {
                    Ok(true)
                } else {
                    Err("更新遇到未知错误".to_string())
                }
            }
            Err(e) => Err(format!("Update error: {}", e)),
        }
    }
}
