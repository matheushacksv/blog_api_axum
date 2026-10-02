# Rust Axum API | training

A simple blog API built to learn Rust.

**Stack**:
- Axum, 
- SQLx, 
- Serde, 
- Tokio, 
- Argon2
- JWT

## Routes

🔑 = requires `Authorization: Bearer <token>`

| method | route | description|
|----|----|-----|
| POST | `/api/users` | register
| POST | `api/users/login` | login, return token |
| GET | `api/users/me` | 🔑 current user |
| GET |  `api/articles?limit=&offset=` | 🔑 list (paginated) |
| POST | `api/articles` | 🔑 create |
| GET | `api/articles/{id}` | 🔑 get one article | 
| PATCH | `api/articles/{id}` | 🔑 update (author only) |
| DELETE | `api/articles/{id}` | 🔑 delete (author only) |
