use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use super::password;
use crate::{
    auth::{self, Claims},
    errors::Error,
    state::AppState,
    users::model::{LoginResponse, LoginUser, NewUser, UpdatePassword, UpdateUser, User},
};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), Error> {
    // return error case password < 8
    validate_password_length(&body.password)?;

    let email = body.email.trim().to_lowercase();
    // return error if email empty
    if email.is_empty() {
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
    let user = get_user_from_db(&state.db, claims.sub).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user(
    claims: Claims,
    State(state): State<AppState>,
    Json(body): Json<UpdateUser>,
) -> Result<(StatusCode, Json<User>), Error> {
    // if name is empty
    if let Some(name) = &body.name {
        if name.trim().is_empty() {
            return Err(Error::ValidationError);
        }
    };

    let email = body.email.as_deref().map(|e| e.trim().to_lowercase());
    // if email is empty
    if let Some(email) = &email {
        if email.is_empty() {
            return Err(Error::ValidationError);
        }
    };

    let user = sqlx::query_as!(
        User,
        "UPDATE users
        SET name = COALESCE($1, name),
            email = COALESCE($2, email)
        WHERE id = $3
        RETURNING id, name, email",
        body.name.as_deref().map(str::trim),
        email.as_deref(),
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => Error::Conflict,
        e => Error::Sqlx(e),
    })?
    .ok_or(Error::NotFound)?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_password(
    claims: Claims,
    State(state): State<AppState>,
    Json(body): Json<UpdatePassword>,
) -> Result<StatusCode, Error> {
    validate_password_length(&body.new_password)?;

    let user_password_hash =
        sqlx::query_scalar!("SELECT password_hash FROM users WHERE id = $1", claims.sub)
            .fetch_optional(&state.db)
            .await?
            .ok_or(Error::NotFound)?;

    if !password::verify(&body.old_password, &user_password_hash)? {
        return Err(Error::ValidationError);
    }

    let new_password_hash = password::hash(&body.new_password)?;

    sqlx::query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        new_password_hash,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// Helpers
async fn get_user_from_db(pool: &PgPool, id: uuid::Uuid) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users
        WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(Error::NotFound)
}

fn validate_password_length(password: &String) -> Result<(), Error> {
    if password.len() < (8 as usize) {
        return Err(Error::ValidationError);
    };

    Ok(())
}
