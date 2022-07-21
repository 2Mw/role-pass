use rusqlite::Connection;

use crate::{
    dao::user::UserDao,
    models::user::{User, UserPasswords},
};

pub struct UserService {
    store: UserDao,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            store: UserDao::new(),
        }
    }

    /// 创建三个表
    /// 1. users (id, name, password, create_time, last_login, login_times)
    /// 2. user_password (id, uid, role, account, password, create_time, update_time, update_count, login_url, tips)
    pub fn create_user_tables(&self, db: &Connection) -> Result<bool, String> {
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

    /// 创建用户
    pub fn create_user(&self, db: &Connection, name: String, pass: String) -> Result<bool, String> {
        if name.chars().count() < 4 {
            return Err("用户名长度需要大于3个字符".to_string());
        }
        if pass.chars().count() < 8 {
            return Err("密码长度需要大于8个字符".to_string());
        }
        self.store.create_user(db, &name, &pass)
    }

    /// 通过用户ID获取其所有角色
    pub fn get_roles_by_id(
        &self,
        db: &Connection,
        id: usize,
    ) -> Result<Vec<UserPasswords>, String> {
        self.store.get_roles_by_id(db, id)
    }

    /// 插入一条账号信息
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
        if role.chars().count() == 0 {
            return Err("角色不能为空".to_string());
        }
        if account.chars().count() == 0 {
            return Err("账号信息不能为空".to_string());
        }
        if pass.chars().count() == 0 {
            return Err("账号密码不能为空".to_string());
        }

        self.store
            .insert_account(db, uid, role, account, pass, login_url, tip, upass)
    }

    /// 通过用户ID获取其所有账号信息
    pub fn get_accounts_by_id(
        &self,
        db: &Connection,
        id: usize,
        key: &String,
    ) -> Result<Vec<UserPasswords>, String> {
        self.store.get_accounts_by_id(db, id, key)
    }

    /// 通过密码 id 来删除一条项目
    pub fn del_accounts_by_id(&self, db: &Connection, id: usize) -> Result<bool, String> {
        self.store.del_accounts_by_id(db, id)
    }

    /// 通过密码 id 来更新一条项目
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
        if account.chars().count() == 0 {
            return Err("账号信息不能为空".to_string());
        }
        if pass.chars().count() == 0 {
            return Err("账号密码不能为空".to_string());
        }
        self.store.update_accounts_by_id(db, id, account, pass, login_url, tip, upass)
    }
}
