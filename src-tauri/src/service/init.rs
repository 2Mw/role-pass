use rusqlite::Connection;

use crate::dao::init as dao;

pub struct InitService {
    store: dao::InitDao,
}

impl InitService {
    pub fn new() -> InitService {
        InitService {
            store: dao::InitDao::new(),
        }
    }

    /// Initialize for `global_config` table
    pub fn init_global_config(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        self.store.init_global_config(conn)
    }

    /// 检查是否设置程序锁
    pub fn if_app_password_set(&self, conn: &Connection) -> Result<bool, rusqlite::Error> {
        self.store.if_app_password_set(conn)
    }

    /// 设置程序锁
    pub fn set_app_password(&self, db: &Connection, pass: String) -> Result<bool, rusqlite::Error> {
        if pass.len() < 8 {
            return Ok(false);
        }
        self.store.set_app_password(db, pass)
    }

    /// 设置程序锁
    pub fn valid_app_password(&self, db: &Connection, pass: String) -> Result<bool, rusqlite::Error> {
        if pass.len() < 8 {
            return Ok(false);
        }
        self.store.valid_app_password(db, pass)
    }
}
