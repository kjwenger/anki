use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Anki Web App API",
        version = "0.1.0",
        description = "REST API for Anki spaced repetition flashcard system

## Authentication

Most endpoints require a JWT token obtained from `/api/v1/auth/login`.
Include the token in the `Authorization` header as `Bearer <token>`.

## Quick Start

1. Register: `POST /api/v1/auth/register`
   ```json
   {\"username\": \"alice\", \"password\": \"password123\"}
   ```

2. Login: `POST /api/v1/auth/login`
   ```json
   {\"username\": \"alice\", \"password\": \"password123\"}
   ```
   Response contains JWT token.

3. Create deck: `POST /api/v1/decks`
   ```bash
   curl -H \"Authorization: Bearer <token>\" \\
        -H \"Content-Type: application/json\" \\
        -d '{\"name\": \"Spanish\"}' \\
        http://localhost:8080/api/v1/decks
   ```

4. Create note: `POST /api/v1/notes`
   ```bash
   curl -H \"Authorization: Bearer <token>\" \\
        -H \"Content-Type: application/json\" \\
        -d '{\"deck_id\": 1, \"notetype_id\": 1, \"fields\": [\"hola\", \"hello\"], \"tags\": [\"vocab\"]}' \\
        http://localhost:8080/api/v1/notes
   ```

## Available Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login (returns JWT token)
- `GET /api/v1/auth/me` - Get current user info 🔒
- `POST /api/v1/auth/logout` - Logout 🔒

### Collections
- `GET /api/v1/collection/info` - Get collection info 🔒
- `POST /api/v1/collection/close` - Close collection 🔒

### Decks
- `GET /api/v1/decks` - Get deck tree 🔒
- `POST /api/v1/decks` - Create deck 🔒
- `GET /api/v1/decks/{id}` - Get deck by ID 🔒
- `DELETE /api/v1/decks/{id}` - Delete deck 🔒

### Notes
- `POST /api/v1/notes` - Create note (auto-generates cards) 🔒
- `GET /api/v1/notes/{id}` - Get note by ID 🔒
- `PUT /api/v1/notes/{id}` - Update note 🔒
- `DELETE /api/v1/notes/{id}` - Delete note (and cards) 🔒
- `GET /api/v1/notes/{id}/cards` - Get cards for note 🔒

### Health
- `GET /health` - Health check
- `GET /api/v1/info` - Server info

🔒 = Requires Authentication (JWT token)
",
        contact(
            name = "Anki Development",
            url = "https://github.com/ankitects/anki"
        ),
        license(
            name = "AGPL-3.0",
            url = "https://www.gnu.org/licenses/agpl-3.0.en.html"
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "collection", description = "Collection management"),
        (name = "decks", description = "Deck management"),
        (name = "notes", description = "Note management"),
        (name = "health", description = "Health check endpoints")
    )
)]
pub struct ApiDoc;
