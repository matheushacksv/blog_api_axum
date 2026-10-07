#[derive(serde::Serialize)]
pub struct ArticleList {
    pub articles: Vec<Article>,
    pub total: i64,
}

#[derive(serde::Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub body: String,
}

#[derive(serde::Deserialize)]
pub struct Filter {
    pub q: Option<String>,
    pub author_id: Option<uuid::Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(serde::Serialize)]
pub struct Article {
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub title: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
