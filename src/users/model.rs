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

#[derive(serde::Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}

// * model user
//
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
}
