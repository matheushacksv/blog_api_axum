use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    articles::model::{Article, ArticleList, NewArticle, Pagination, UpdateArticle},
    auth::Claims,
    errors::Error,
    state::AppState,
};

pub async fn create(
    claims: Claims,
    State(state): State<AppState>,
    Json(body): Json<NewArticle>,
) -> Result<(StatusCode, Json<Article>), Error> {
    if body.title.trim().is_empty() || body.body.trim().is_empty() {
        return Err(Error::ValidationError);
    }

    let article = sqlx::query_as!(
        Article,
        "INSERT INTO articles (author_id, title, body)
        VALUES ($1, $2, $3)
        RETURNING id, author_id, title, body, created_at, updated_at",
        claims.sub,
        body.title.trim(),
        body.body.trim()
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(article)))
}

pub async fn get_one(
    _claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<Article>), Error> {
    let article = sqlx::query_as!(
        Article,
        "SELECT id, author_id, title, body, created_at, updated_at
        FROM articles WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(Error::NotFound)?;

    Ok((StatusCode::OK, Json(article)))
}

pub async fn list(
    _claims: Claims,
    State(state): State<AppState>,
    Query(params): Query<Pagination>,
) -> Result<(StatusCode, Json<ArticleList>), Error> {
    // set limit or offset
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    // get articles
    let articles = sqlx::query_as!(
        Article,
        "SELECT id, author_id, title, body, created_at, updated_at
        FROM articles
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&state.db)
    .await?;

    // count the total
    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM articles
        "#
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(ArticleList { articles, total })))
}

pub async fn update(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateArticle>,
) -> Result<(StatusCode, Json<Article>), Error> {
    // if exist title, return error if is empty
    if let Some(title) = &body.title {
        if title.trim().is_empty() {
            return Err(Error::ValidationError);
        }
    };

    // if exist body, return error if is empty
    if let Some(body) = &body.body {
        if body.trim().is_empty() {
            return Err(Error::ValidationError);
        }
    };

    // get the author_id for id in the request
    ensure_author(&state.db, id, claims.sub).await?;

    // update database
    let article = sqlx::query_as!(
        Article,
        "UPDATE articles
        SET title = COALESCE($1, title),
            body = COALESCE($2, body),
            updated_at = now()
        WHERE id = $3
        RETURNING id, author_id, title, body, created_at, updated_at",
        body.title.as_deref().map(str::trim),
        body.body.as_deref().map(str::trim),
        id,
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::OK, Json(article)))
}

pub async fn remove(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(StatusCode, ()), Error> {
    ensure_author(&state.db, id, claims.sub).await?;

    sqlx::query!("DELETE FROM articles WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

// Helper
async fn ensure_author(db: &PgPool, article_id: Uuid, user_id: Uuid) -> Result<(), Error> {
    let author_id = sqlx::query_scalar!("SELECT author_id FROM articles WHERE id = $1", article_id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::NotFound)?;

    if author_id != user_id {
        return Err(Error::Forbidden);
    };

    Ok(())
}
