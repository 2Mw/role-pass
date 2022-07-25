#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lazy_static::lazy_static;

mod util;

mod dao;

mod models;

mod service;
use models::user::{User, UserPasswords};
use service::{InitService, UserService};

use std::{fs::create_dir, path::PathBuf};

use rusqlite::{Connection, Error};
use tauri::api::path::home_dir;

// Lazy initialize
lazy_static! {
    static ref INIT_SEVICE: InitService = InitService::new();
    static ref USER_SEVICE: UserService = UserService::new();
}


/**
 * TODO: 
 * 1. 实现导入功能
 * 2. 实现修改密码的功能(高危)
 * 3. 分清楚导出和备份的区别
 */

static mut DB: Option<&mut Connection> = None;

/// init 用于初始化配置文件和目录
/// 1. 初始化 SQLite 的文件目录
/// 2. 初始化并且读取配置
fn init() {
    // conf 用于存储配置文件
    // store 用于存储sqlite文件
    let sub_dirs = vec!["conf", "store"];
    let home = home_dir().expect("Getting home dir failed");
    let home = home.join("role-pass");
    if home.exists() {
        // TODO 已存在，读取配置信息
    } else {
        // 第一次使用，需要创建目录
        create_dir(&home).expect("Create program dir failed.");
    }
    // 检查子目录是否全部存在
    for dir in &sub_dirs {
        let d = home.join(dir);
        if !d.exists() {
            // 子目录不存在则创建
            let e = create_dir(d);
            if let Err(err) = e {
                panic!("Create {} dir failed, {}", dir, err);
            }
        }
    }
    // 1. 初始化 SQLite
    let res = init_sqlite(home.join(sub_dirs[1])).expect("Open sqlite failed.");
    dbg!(res);
}

/// 初始化 SQLite 的文件目录
///
/// 使用的包 https://github.com/rusqlite/rusqlite
fn init_sqlite(path: PathBuf) -> Result<&'static Connection, Error> {
    let path = path.join(path.join("store.db"));
    let db;
    unsafe {
        let conn = Box::new(Connection::open(path)?);
        DB = Some(Box::leak(conn));
        if let Some(x) = &DB {
            db = x;
        } else {
            panic!("Initial sqlite failed.")
        }
    }

    // 初始化表 global_config
    let res: Result<usize, rusqlite::Error> =
        db.query_row("SELECT ID from global_config limit 1;", [], |r| r.get(0));

    if let Err(e) = res {
        // 不存在表 global_config
        println!("err:{}", e);
        INIT_SEVICE
            .init_global_config(&db)
            .expect("Initial global_config table failed");
    } else {
        // 添加初始化值
        let _ = db.execute("UPDATE global_config SET run_count=run_count+1, last_run=datetime('now','localtime') where ID = 1;", ());
    }

    // 初始化用户相关表
    USER_SEVICE.create_user_tables(&db).unwrap();

    Ok(db)
}

// ======================================   About Init  ==============================================

/// Check if app password is set.
#[tauri::command]
fn if_app_password_set() -> bool {
    unsafe {
        if let Some(db) = &DB {
            // db.execute("SELECT app_password from global_config where ID = 1", ())
            if let Ok(e) = INIT_SEVICE.if_app_password_set(db) {
                return e;
            }
        }
        false
    }
}

/// Set app password
#[tauri::command]
fn set_app_password(pass: String) -> bool {
    unsafe {
        if let Some(db) = &DB {
            if let Ok(e) = INIT_SEVICE.set_app_password(db, pass) {
                return e;
            }
        }
        false
    }
}

/// valid app password
#[tauri::command]
fn valid_app_password(pass: String) -> bool {
    unsafe {
        if let Some(db) = &DB {
            if let Ok(e) = INIT_SEVICE.valid_app_password(db, pass) {
                return e;
            }
        }
        false
    }
}

// ======================================   About Users  ==============================================

#[tauri::command]
fn query_users() -> Result<Vec<User>, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.query_users(db)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn login(id: usize, pass: String) -> Result<bool, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.login(db, id, pass)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn create_user(name: String, pass: String) -> Result<bool, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.create_user(db, name, pass)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn get_roles_by_id(id: usize) -> Result<Vec<UserPasswords>, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.get_roles_by_id(db, id)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn insert_account(
    uid: usize,
    role: String,
    account: String,
    pass: String,
    login_url: String,
    tip: String,
    upass: String,
) -> Result<bool, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.insert_account(db, uid, role, account, pass, login_url, tip, upass)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn get_accounts_by_id(id: usize, key: String) -> Result<Vec<UserPasswords>, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.get_accounts_by_id(db, id, &key)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn del_accounts_by_id(id: usize) -> Result<bool, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.del_accounts_by_id(db, id)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

#[tauri::command]
fn update_accounts_by_id(
    id: usize,
    account: String,
    pass: String,
    login_url: String,
    tip: String,
    upass: String,
) -> Result<bool, String> {
    unsafe {
        if let Some(db) = &DB {
            USER_SEVICE.update_accounts_by_id(db, id, account, pass, login_url, tip, upass)
        } else {
            Err("Datasource connection error".to_string())
        }
    }
}

// ======================================   Main  ==============================================

fn main() {
    init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_app_password,
            if_app_password_set,
            valid_app_password,
            // Users
            query_users,
            login,
            create_user,
            get_roles_by_id,
            insert_account,
            get_accounts_by_id,
            del_accounts_by_id,
            update_accounts_by_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
