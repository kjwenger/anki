// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::json;
use serde_json::Value;

pub fn openapi_spec() -> Value {
    json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Anki Web App API",
            "version": "0.1.0",
            "description": "REST API for Anki spaced repetition flashcard system.\n\n## Authentication\n\nMost endpoints require a JWT token obtained from `POST /api/v1/auth/login`.\nInclude the token in the `Authorization` header as `Bearer <token>`.",
            "contact": {
                "name": "Anki Development",
                "url": "https://github.com/ankitects/anki"
            },
            "license": {
                "name": "AGPL-3.0",
                "url": "https://www.gnu.org/licenses/agpl-3.0.en.html"
            }
        },
        "servers": [
            { "url": "/", "description": "Current server" }
        ],
        "tags": [
            { "name": "auth", "description": "Authentication endpoints" },
            { "name": "collection", "description": "Collection management" },
            { "name": "decks", "description": "Deck management" },
            { "name": "notes", "description": "Note management" },
            { "name": "cards", "description": "Card management" },
            { "name": "health", "description": "Health check endpoints" }
        ],
        "paths": {
            "/api/v1/auth/register": {
                "post": {
                    "tags": ["auth"],
                    "summary": "Register a new user",
                    "operationId": "register",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/RegisterRequest" }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "User registered successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/AuthResponse" }
                                }
                            }
                        },
                        "400": { "$ref": "#/components/responses/BadRequest" },
                        "409": {
                            "description": "Username already exists",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/ErrorResponse" }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/auth/login": {
                "post": {
                    "tags": ["auth"],
                    "summary": "Login and obtain JWT token",
                    "operationId": "login",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/LoginRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/AuthResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/auth/me": {
                "get": {
                    "tags": ["auth"],
                    "summary": "Get current user info",
                    "operationId": "me",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Current user info",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/UserInfo" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/auth/profile": {
                "get": {
                    "tags": ["auth"],
                    "summary": "Get current user profile (alias for /me)",
                    "operationId": "profile",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Current user info",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/UserInfo" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/auth/logout": {
                "post": {
                    "tags": ["auth"],
                    "summary": "Logout current user",
                    "operationId": "logout",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Logged out successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "message": { "type": "string", "example": "Logged out successfully" }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/collection/info": {
                "get": {
                    "tags": ["collection"],
                    "summary": "Get collection info",
                    "operationId": "getCollectionInfo",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Collection info",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/CollectionInfo" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/collection/close": {
                "post": {
                    "tags": ["collection"],
                    "summary": "Close the collection",
                    "operationId": "closeCollection",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Collection closed",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "message": { "type": "string", "example": "Collection closed successfully" }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/decks": {
                "get": {
                    "tags": ["decks"],
                    "summary": "Get deck tree",
                    "operationId": "getDeckTree",
                    "security": [{ "bearerAuth": [] }],
                    "responses": {
                        "200": {
                            "description": "Deck tree",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "decks": {
                                                "type": "array",
                                                "items": { "$ref": "#/components/schemas/DeckTreeNode" }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                },
                "post": {
                    "tags": ["decks"],
                    "summary": "Create a new deck",
                    "operationId": "createDeck",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/CreateDeckRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Deck created",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "message": { "type": "string", "example": "Deck 'Spanish' created" },
                                            "id": { "type": "integer", "format": "int64" }
                                        }
                                    }
                                }
                            }
                        },
                        "400": { "$ref": "#/components/responses/BadRequest" },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/decks/{id}": {
                "get": {
                    "tags": ["decks"],
                    "summary": "Get deck by ID",
                    "operationId": "getDeck",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{ "$ref": "#/components/parameters/DeckId" }],
                    "responses": {
                        "200": {
                            "description": "Deck details",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/Deck" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                },
                "delete": {
                    "tags": ["decks"],
                    "summary": "Delete a deck",
                    "operationId": "deleteDeck",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{ "$ref": "#/components/parameters/DeckId" }],
                    "responses": {
                        "200": {
                            "description": "Deck deleted",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "message": { "type": "string", "example": "Deck deleted successfully" }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/notes": {
                "post": {
                    "tags": ["notes"],
                    "summary": "Create a new note (auto-generates cards)",
                    "operationId": "createNote",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/CreateNoteRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Note created",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "note_id": { "type": "integer", "format": "int64" },
                                            "message": { "type": "string", "example": "Note created successfully (1 cards generated)" }
                                        }
                                    }
                                }
                            }
                        },
                        "400": { "$ref": "#/components/responses/BadRequest" },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/notes/{id}": {
                "get": {
                    "tags": ["notes"],
                    "summary": "Get note by ID",
                    "operationId": "getNote",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{ "$ref": "#/components/parameters/NoteId" }],
                    "responses": {
                        "200": {
                            "description": "Note details",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/Note" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                },
                "put": {
                    "tags": ["notes"],
                    "summary": "Update a note",
                    "operationId": "updateNote",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{ "$ref": "#/components/parameters/NoteId" }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/UpdateNoteRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Note updated",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "message": { "type": "string", "example": "Note updated successfully" }
                                        }
                                    }
                                }
                            }
                        },
                        "400": { "$ref": "#/components/responses/BadRequest" },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                },
                "delete": {
                    "tags": ["notes"],
                    "summary": "Delete a note and its cards",
                    "operationId": "deleteNote",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{ "$ref": "#/components/parameters/NoteId" }],
                    "responses": {
                        "200": {
                            "description": "Note deleted",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean", "example": true },
                                            "message": { "type": "string", "example": "Note deleted successfully (1 cards removed)" }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/notes/{id}/cards": {
                "get": {
                    "tags": ["notes"],
                    "summary": "Get cards for a note",
                    "operationId": "getNoteCards",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [{ "$ref": "#/components/parameters/NoteId" }],
                    "responses": {
                        "200": {
                            "description": "Card IDs for the note",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "card_ids": {
                                                "type": "array",
                                                "items": { "type": "integer", "format": "int64" }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/cards/{id}": {
                "get": {
                    "tags": ["cards"],
                    "summary": "Get a card by ID",
                    "operationId": "getCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Card information",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/CardInfo" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                },
                "put": {
                    "tags": ["cards"],
                    "summary": "Update a card",
                    "operationId": "updateCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/UpdateCardRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Card updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MessageResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                },
                "delete": {
                    "tags": ["cards"],
                    "summary": "Delete a card",
                    "operationId": "deleteCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Card deleted successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MessageResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/cards/{id}/flag": {
                "post": {
                    "tags": ["cards"],
                    "summary": "Flag a card",
                    "operationId": "flagCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/FlagCardRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Card flagged successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MessageResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/cards/{id}/suspend": {
                "post": {
                    "tags": ["cards"],
                    "summary": "Suspend a card",
                    "operationId": "suspendCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Card suspended successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MessageResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/cards/{id}/unsuspend": {
                "post": {
                    "tags": ["cards"],
                    "summary": "Unsuspend a card",
                    "operationId": "unsuspendCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Card unsuspended successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MessageResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/cards/{id}/bury": {
                "post": {
                    "tags": ["cards"],
                    "summary": "Bury a card",
                    "operationId": "buryCard",
                    "security": [{ "bearerAuth": [] }],
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": true,
                            "schema": { "type": "integer", "format": "int64" },
                            "description": "Card ID"
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Card buried successfully",
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/MessageResponse" }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" },
                        "404": { "$ref": "#/components/responses/NotFound" }
                    }
                }
            },
            "/api/v1/cards/batch": {
                "post": {
                    "tags": ["cards"],
                    "summary": "Get multiple cards by IDs",
                    "operationId": "batchGetCards",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/BatchGetCardsRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Cards retrieved successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "cards": {
                                                "type": "array",
                                                "items": { "$ref": "#/components/schemas/CardInfo" }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/api/v1/cards/batch-update": {
                "post": {
                    "tags": ["cards"],
                    "summary": "Update multiple cards",
                    "operationId": "batchUpdateCards",
                    "security": [{ "bearerAuth": [] }],
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": { "$ref": "#/components/schemas/BatchUpdateCardsRequest" }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Cards updated successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean" },
                                            "message": { "type": "string" },
                                            "updated_count": { "type": "integer" }
                                        }
                                    }
                                }
                            }
                        },
                        "401": { "$ref": "#/components/responses/Unauthorized" }
                    }
                }
            },
            "/health": {
                "get": {
                    "tags": ["health"],
                    "summary": "Health check",
                    "operationId": "healthCheck",
                    "responses": {
                        "200": {
                            "description": "Server is healthy",
                            "content": {
                                "text/plain": {
                                    "schema": { "type": "string", "example": "OK" }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/info": {
                "get": {
                    "tags": ["health"],
                    "summary": "Get server info",
                    "operationId": "serverInfo",
                    "responses": {
                        "200": {
                            "description": "Server information",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "success": { "type": "boolean" },
                                            "data": {
                                                "type": "object",
                                                "properties": {
                                                    "name": { "type": "string", "example": "Anki Web App" },
                                                    "version": { "type": "string" },
                                                    "status": { "type": "string", "example": "running" },
                                                    "features": {
                                                        "type": "object",
                                                        "properties": {
                                                            "authentication": { "type": "boolean" },
                                                            "api": { "type": "boolean" },
                                                            "ui": { "type": "boolean" }
                                                        }
                                                    },
                                                    "message": { "type": "string" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "securitySchemes": {
                "bearerAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT",
                    "description": "JWT token obtained from POST /api/v1/auth/login"
                }
            },
            "parameters": {
                "DeckId": {
                    "name": "id",
                    "in": "path",
                    "required": true,
                    "description": "Deck ID",
                    "schema": { "type": "integer", "format": "int64" }
                },
                "NoteId": {
                    "name": "id",
                    "in": "path",
                    "required": true,
                    "description": "Note ID",
                    "schema": { "type": "integer", "format": "int64" }
                }
            },
            "schemas": {
                "RegisterRequest": {
                    "type": "object",
                    "required": ["username", "password"],
                    "properties": {
                        "username": { "type": "string", "maxLength": 50, "example": "alice" },
                        "password": { "type": "string", "minLength": 8, "example": "password123" },
                        "email": { "type": "string", "format": "email", "nullable": true, "example": "alice@example.com" }
                    }
                },
                "LoginRequest": {
                    "type": "object",
                    "required": ["username", "password"],
                    "properties": {
                        "username": { "type": "string", "example": "alice" },
                        "password": { "type": "string", "example": "password123" }
                    }
                },
                "AuthResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean", "example": true },
                        "data": {
                            "type": "object",
                            "properties": {
                                "token": { "type": "string", "example": "eyJhbGciOiJIUzI1NiIs..." },
                                "user": { "$ref": "#/components/schemas/UserInfo" }
                            }
                        },
                        "error": { "type": "string", "nullable": true }
                    }
                },
                "UserInfo": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer", "format": "int64" },
                        "username": { "type": "string" },
                        "email": { "type": "string", "nullable": true }
                    }
                },
                "CollectionInfo": {
                    "type": "object",
                    "properties": {
                        "user_id": { "type": "integer", "format": "int64" },
                        "username": { "type": "string" },
                        "backend_active": { "type": "boolean" },
                        "message": { "type": "string" }
                    }
                },
                "DeckTreeNode": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer", "format": "int64" },
                        "name": { "type": "string" },
                        "collapsed": { "type": "boolean" },
                        "children": {
                            "type": "array",
                            "items": { "$ref": "#/components/schemas/DeckTreeNode" }
                        }
                    }
                },
                "Deck": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer", "format": "int64" },
                        "name": { "type": "string" }
                    }
                },
                "CreateDeckRequest": {
                    "type": "object",
                    "required": ["name"],
                    "properties": {
                        "name": { "type": "string", "example": "Spanish" }
                    }
                },
                "CreateNoteRequest": {
                    "type": "object",
                    "required": ["deck_id", "notetype_id", "fields", "tags"],
                    "properties": {
                        "deck_id": { "type": "integer", "format": "int64", "example": 1 },
                        "notetype_id": { "type": "integer", "format": "int64", "example": 1 },
                        "fields": {
                            "type": "array",
                            "items": { "type": "string" },
                            "example": ["hola", "hello"]
                        },
                        "tags": {
                            "type": "array",
                            "items": { "type": "string" },
                            "example": ["vocab"]
                        }
                    }
                },
                "UpdateNoteRequest": {
                    "type": "object",
                    "required": ["fields", "tags"],
                    "properties": {
                        "fields": {
                            "type": "array",
                            "items": { "type": "string" },
                            "example": ["hola", "hello (greeting)"]
                        },
                        "tags": {
                            "type": "array",
                            "items": { "type": "string" },
                            "example": ["vocab", "greetings"]
                        }
                    }
                },
                "Note": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer", "format": "int64" },
                        "fields": {
                            "type": "array",
                            "items": { "type": "string" }
                        },
                        "tags": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    }
                },
                "CardInfo": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer", "format": "int64" },
                        "note_id": { "type": "integer", "format": "int64" },
                        "deck_id": { "type": "integer", "format": "int64" },
                        "ordinal": { "type": "integer", "format": "uint16" },
                        "card_type": { "type": "integer", "format": "uint8", "description": "0=New, 1=Learn, 2=Review, 3=Relearn" },
                        "queue": { "type": "integer", "format": "int8", "description": "Card queue state" },
                        "due": { "type": "integer", "format": "int32" },
                        "interval": { "type": "integer", "format": "uint32" },
                        "ease_factor": { "type": "integer", "format": "uint16" },
                        "reps": { "type": "integer", "format": "uint32" },
                        "lapses": { "type": "integer", "format": "uint32" },
                        "flags": { "type": "integer", "format": "uint8", "description": "0=None, 1=Red, 2=Orange, 3=Green, 4=Blue" }
                    }
                },
                "UpdateCardRequest": {
                    "type": "object",
                    "properties": {
                        "deck_id": { "type": "integer", "format": "int64", "nullable": true },
                        "due": { "type": "integer", "format": "int32", "nullable": true },
                        "flags": { "type": "integer", "format": "uint8", "nullable": true }
                    }
                },
                "FlagCardRequest": {
                    "type": "object",
                    "required": ["flag"],
                    "properties": {
                        "flag": {
                            "type": "integer",
                            "format": "uint8",
                            "minimum": 0,
                            "maximum": 4,
                            "description": "0=None, 1=Red, 2=Orange, 3=Green, 4=Blue",
                            "example": 1
                        }
                    }
                },
                "BatchGetCardsRequest": {
                    "type": "object",
                    "required": ["card_ids"],
                    "properties": {
                        "card_ids": {
                            "type": "array",
                            "items": { "type": "integer", "format": "int64" },
                            "example": [1, 2, 3]
                        }
                    }
                },
                "BatchUpdateCardsRequest": {
                    "type": "object",
                    "required": ["updates"],
                    "properties": {
                        "updates": {
                            "type": "array",
                            "items": { "$ref": "#/components/schemas/BatchCardUpdate" }
                        }
                    }
                },
                "BatchCardUpdate": {
                    "type": "object",
                    "required": ["card_id"],
                    "properties": {
                        "card_id": { "type": "integer", "format": "int64" },
                        "deck_id": { "type": "integer", "format": "int64", "nullable": true },
                        "due": { "type": "integer", "format": "int32", "nullable": true },
                        "flags": { "type": "integer", "format": "uint8", "nullable": true }
                    }
                },
                "MessageResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean", "example": true },
                        "message": { "type": "string", "example": "Operation successful" }
                    }
                },
                "ErrorResponse": {
                    "type": "object",
                    "properties": {
                        "success": { "type": "boolean", "example": false },
                        "data": { "nullable": true },
                        "error": { "type": "string", "example": "Error description" }
                    }
                }
            },
            "responses": {
                "BadRequest": {
                    "description": "Bad request",
                    "content": {
                        "application/json": {
                            "schema": { "$ref": "#/components/schemas/ErrorResponse" }
                        }
                    }
                },
                "Unauthorized": {
                    "description": "Missing or invalid authentication token",
                    "content": {
                        "application/json": {
                            "schema": { "$ref": "#/components/schemas/ErrorResponse" }
                        }
                    }
                },
                "NotFound": {
                    "description": "Resource not found",
                    "content": {
                        "application/json": {
                            "schema": { "$ref": "#/components/schemas/ErrorResponse" }
                        }
                    }
                }
            }
        }
    })
}
