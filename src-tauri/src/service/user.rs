use rusqlite::Connection;

use crate::{dao::user::UserDao, models::user::User};

pub struct UserService {
    store: UserDao
}

impl UserService {
    pub fn new() -> Self {
        UserService { store: UserDao::new() }
    }

    /// 创建三个表
    /// 1. users (id, name, password, create_time, last_login, login_times)
    /// 2. user_roles (id, uid, role, tips)
    /// 3. user_password (id, uid, rid, account, password, create_time, update_time, update_count, login_url, tips)
    pub fn create_user_tables(&self, db: &Connection)  -> Result<bool, String> {
        self.store.create_user_tables(&db)
    }

    /// 查询所有的用户
    pub fn query_users(&self, db: &Connection) -> Result<Vec<User>, String> {
        self.store.query_users(db)
    }

    /// 用户登录
    pub fn login(&self, db: &Connection, id: usize, pass: String) -> Result<bool, String> {
        self.store.login(db, id, pass)
    }
}