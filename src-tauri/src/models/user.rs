
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
pub struct Role {
    pub id: usize,
    pub uid: usize,
    pub role: String,
    pub tip: String
}