// * Schemas
//
#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

// * model user
// 
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
}
