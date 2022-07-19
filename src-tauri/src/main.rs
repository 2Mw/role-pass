#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod util;

use std::{fs::create_dir, path::{PathBuf}};

use rusqlite::{Connection, Error};
use tauri::api::path::home_dir;

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
fn init_sqlite(path: PathBuf) -> Result<Connection, Error>{
	let path = path.join(path.join("store.db"));
	let db = Connection::open(path)?;
	Ok(db)
}

fn main() {
    init();
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
