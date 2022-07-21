
#[derive(serde::Serialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub password: String,
    pub create_time: usize,
    pub last_login: usize,
    pub login_times: usize
}

#[derive(serde::Serialize)]
pub struct UserPasswords {
    pub id: usize,
    pub uid: usize,
    pub role: String,
    pub account: String,
    pub password: String,
    pub create_time: usize,
    pub last_update: usize,
    pub update_count: usize,
    pub login_url: String,
    pub tip: String
}