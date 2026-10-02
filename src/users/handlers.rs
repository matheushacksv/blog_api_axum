use axum::{Json, extract::State, http::StatusCode};

use super::password;
use crate::{
    auth::{self, Claims},
    errors::Error,
    state::AppState,
    users::model::{LoginResponse, LoginUser, NewUser, User},
};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), Error> {
    // return error case password < 8
    if body.password.len() < (8 as usize) {
        return Err(Error::ValidationError);
    }

    // return error if email empty
    if body.email.is_empty() {
        return Err(Error::ValidationError);
    }

    // hash password
    let password_hash = password::hash(&body.password)?;

    // create user in database
    let user = sqlx::query_as!(
        User,
        "
        INSERT INTO users (name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, name, email",
        body.name,
        body.email.trim().to_lowercase(),
        password_hash,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => Error::Conflict,
        e => Error::Sqlx(e),
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginUser>,
) -> Result<(StatusCode, Json<LoginResponse>), Error> {
    // get from database
    let row = sqlx::query!(
        "SELECT id, name, email, password_hash FROM users WHERE email = $1",
        body.email.trim().to_lowercase()
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::Unauthorized)?;

    // check password
    if !password::verify(&body.password, &row.password_hash)? {
        return Err(Error::Unauthorized);
    }

    // instanced user
    let user = User {
        id: row.id,
        name: row.name,
        email: row.email,
    };

    // create token
    let token = auth::create_token(user.id, state.config.jwt_secret())?;

    Ok((StatusCode::OK, Json(LoginResponse { user, token })))
}

pub async fn me(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<User>), Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users
        WHERE id = $1",
        claims.sub,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok((StatusCode::OK, Json(user)))
}
