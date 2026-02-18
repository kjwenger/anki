# Cards API - Quick Reference

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

All endpoints require JWT token in Authorization header:

```
Authorization: Bearer <token>
```

---

## Endpoints

### Individual Card Operations

#### Get Card

```http
GET /cards/{id}
```

**Response:**

```json
{
    "id": 1,
    "note_id": 1,
    "deck_id": 1,
    "ordinal": 0,
    "card_type": 0,
    "queue": 0,
    "due": 1,
    "interval": 0,
    "ease_factor": 2500,
    "reps": 0,
    "lapses": 0,
    "flags": 0
}
```

#### Update Card

```http
PUT /cards/{id}
Content-Type: application/json

{
  "deck_id": 2,        // optional
  "due": 5,            // optional
  "flags": 1           // optional: 0-4
}
```

#### Delete Card

```http
DELETE /cards/{id}
```

#### Flag Card

```http
POST /cards/{id}/flag
Content-Type: application/json

{
  "flag": 1  // 0=None, 1=Red, 2=Orange, 3=Green, 4=Blue
}
```

#### Suspend Card

```http
POST /cards/{id}/suspend
```

#### Unsuspend Card

```http
POST /cards/{id}/unsuspend
```

#### Bury Card

```http
POST /cards/{id}/bury
```

### Batch Operations

#### Batch Get Cards

```http
POST /cards/batch
Content-Type: application/json

{
  "card_ids": [1, 2, 3]
}
```

**Response:**

```json
{
  "cards": [
    { "id": 1, ... },
    { "id": 2, ... },
    { "id": 3, ... }
  ]
}
```

#### Batch Update Cards

```http
POST /cards/batch-update
Content-Type: application/json

{
  "updates": [
    {
      "card_id": 1,
      "deck_id": 2,     // optional
      "due": 5,         // optional
      "flags": 1        // optional
    },
    {
      "card_id": 2,
      "flags": 2
    }
  ]
}
```

**Response:**

```json
{
    "success": true,
    "message": "Updated 2 cards successfully",
    "updated_count": 2
}
```

---

## Card Types

- `0` = New
- `1` = Learn
- `2` = Review
- `3` = Relearn

## Queue States

- `0` = New
- `1` = Learn (due is unix timestamp)
- `2` = Review (due is days since creation)
- `3` = DayLearn
- `4` = PreviewRepeat
- `-1` = Suspended
- `-2` = SchedBuried
- `-3` = UserBuried

## Flag Colors

- `0` = None
- `1` = Red
- `2` = Orange
- `3` = Green
- `4` = Blue

---

## Error Responses

All errors return:

```json
{
    "success": false,
    "error": "Error message"
}
```

**HTTP Status Codes:**

- `200` - Success
- `400` - Bad Request
- `401` - Unauthorized (missing/invalid token)
- `404` - Not Found
- `500` - Internal Server Error

---

## Example Workflow

```bash
# 1. Get JWT token
TOKEN=$(curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"pass"}' | jq -r '.data.token')

# 2. Get a card
curl http://localhost:8080/api/v1/cards/1 \
  -H "Authorization: Bearer $TOKEN"

# 3. Flag the card as red
curl -X POST http://localhost:8080/api/v1/cards/1/flag \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"flag":1}'

# 4. Update multiple cards
curl -X POST http://localhost:8080/api/v1/cards/batch-update \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"updates":[{"card_id":1,"flags":2},{"card_id":2,"flags":3}]}'
```

---

## Swagger UI

Interactive API documentation:

```
http://localhost:8080/swagger-ui
```
