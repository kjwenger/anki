// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use serde_json::{json, Value};

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
