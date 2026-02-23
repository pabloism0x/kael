---
name: api-design
description: RESTful API design principles, naming conventions, and best practices. Use when designing or reviewing API endpoints.
---

# API Design Patterns

## Quick Reference

| Method | Action | Idempotent | Safe |
|--------|--------|------------|------|
| GET | Read | Yes | Yes |
| POST | Create | No | No |
| PUT | Replace | Yes | No |
| PATCH | Update | No | No |
| DELETE | Remove | Yes | No |

| Status | Meaning |
|--------|---------|
| 200 | OK (GET, PUT, PATCH) |
| 201 | Created (POST) |
| 204 | No Content (DELETE) |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 409 | Conflict |
| 422 | Unprocessable Entity |
| 429 | Too Many Requests |
| 500 | Internal Server Error |

## URL Design

### Resource Naming

```
# Good: Nouns, plural, lowercase, hyphens
GET    /users
GET    /users/{id}
GET    /users/{id}/orders
GET    /order-items

# Bad: Verbs, singular, mixed case, underscores
GET    /getUser
GET    /User/{id}
GET    /user_orders
POST   /createUser
```

### Hierarchical Resources

```
# Parent-child relationship
GET    /users/{userId}/orders
GET    /users/{userId}/orders/{orderId}
POST   /users/{userId}/orders

# Avoid deep nesting (max 2 levels)
# Bad
GET    /users/{id}/orders/{id}/items/{id}/reviews

# Good: Use query params or flatten
GET    /order-items/{id}/reviews
GET    /reviews?orderId={id}
```

### Query Parameters

```
# Filtering
GET /users?status=active&role=admin

# Pagination
GET /users?page=2&limit=20
GET /users?offset=40&limit=20
GET /users?cursor=abc123&limit=20

# Sorting
GET /users?sort=created_at:desc
GET /users?sort=-created_at,name

# Field selection (sparse fieldsets)
GET /users?fields=id,name,email

# Search
GET /users?q=john
GET /users?search=john@example.com
```

## Request/Response Design

### Request Body

```json
// POST /users
{
  "email": "user@example.com",
  "name": "John Doe",
  "password": "securePassword123"
}

// PATCH /users/{id} - partial update
{
  "name": "Jane Doe"
}

// PUT /users/{id} - full replacement
{
  "email": "user@example.com",
  "name": "Jane Doe",
  "role": "admin"
}
```

### Response Structure

```json
// Single resource
{
  "id": "123",
  "email": "user@example.com",
  "name": "John Doe",
  "createdAt": "2024-01-15T10:30:00Z",
  "updatedAt": "2024-01-20T14:22:00Z"
}

// Collection with pagination
{
  "data": [
    { "id": "1", "name": "Item 1" },
    { "id": "2", "name": "Item 2" }
  ],
  "meta": {
    "total": 100,
    "page": 1,
    "limit": 20,
    "pages": 5
  },
  "links": {
    "self": "/items?page=1",
    "next": "/items?page=2",
    "last": "/items?page=5"
  }
}

// Cursor-based pagination
{
  "data": [...],
  "meta": {
    "hasMore": true,
    "nextCursor": "eyJpZCI6MTAwfQ=="
  }
}
```

### Error Response

```json
// Standard error format
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Request validation failed",
    "details": [
      {
        "field": "email",
        "message": "Invalid email format"
      },
      {
        "field": "password",
        "message": "Password must be at least 8 characters"
      }
    ]
  },
  "requestId": "req-abc123"
}

// Simple error
{
  "error": {
    "code": "NOT_FOUND",
    "message": "User not found"
  }
}
```

## Versioning

### URL Versioning (Recommended)

```
GET /v1/users
GET /v2/users
```

### Header Versioning

```
GET /users
Accept: application/vnd.api+json; version=1
```

### Query Parameter

```
GET /users?version=1
```

## Authentication & Authorization

### Headers

```
# Bearer token
Authorization: Bearer eyJhbGciOiJIUzI1NiIs...

# API Key
X-API-Key: your-api-key

# Basic Auth (avoid for APIs)
Authorization: Basic base64(username:password)
```

### Response for Auth Errors

```json
// 401 Unauthorized - Missing or invalid credentials
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or expired token"
  }
}

// 403 Forbidden - Valid credentials, insufficient permissions
{
  "error": {
    "code": "FORBIDDEN",
    "message": "You don't have permission to access this resource"
  }
}
```

## Rate Limiting

### Headers

```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640000000
Retry-After: 60
```

### 429 Response

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests",
    "retryAfter": 60
  }
}
```

## Bulk Operations

### Batch Create

```
POST /users/batch

{
  "items": [
    { "email": "user1@example.com", "name": "User 1" },
    { "email": "user2@example.com", "name": "User 2" }
  ]
}

// Response
{
  "results": [
    { "status": "created", "id": "123" },
    { "status": "error", "error": { "code": "DUPLICATE", "message": "Email exists" } }
  ]
}
```

### Batch Update

```
PATCH /users/batch

{
  "items": [
    { "id": "123", "name": "Updated Name 1" },
    { "id": "456", "name": "Updated Name 2" }
  ]
}
```

### Batch Delete

```
DELETE /users/batch

{
  "ids": ["123", "456", "789"]
}
```

## HATEOAS (Hypermedia)

```json
{
  "id": "123",
  "name": "Order #123",
  "status": "pending",
  "_links": {
    "self": { "href": "/orders/123" },
    "cancel": { "href": "/orders/123/cancel", "method": "POST" },
    "items": { "href": "/orders/123/items" },
    "customer": { "href": "/users/456" }
  }
}
```

## Webhooks

### Webhook Payload

```json
{
  "id": "evt_123",
  "type": "user.created",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "id": "user_123",
    "email": "new@example.com"
  }
}
```

### Webhook Headers

```
X-Webhook-Signature: sha256=abc123...
X-Webhook-Timestamp: 1640000000
X-Webhook-ID: evt_123
```

## Anti-patterns

### Avoid: Verbs in URLs

```
# Bad
POST /createUser
GET  /getUserById/{id}
POST /deleteUser/{id}

# Good
POST   /users
GET    /users/{id}
DELETE /users/{id}
```

### Avoid: Inconsistent Naming

```
# Bad - Mixed conventions
GET /users/{userId}
GET /orders/{order_id}
GET /Products/{productID}

# Good - Consistent
GET /users/{id}
GET /orders/{id}
GET /products/{id}
```

### Avoid: Leaking Internal IDs

```
# Bad - Sequential IDs expose data
GET /users/1
GET /users/2

# Good - UUIDs or public IDs
GET /users/550e8400-e29b-41d4-a716-446655440000
GET /users/usr_abc123
```

### Avoid: Overloading POST

```
# Bad - Using POST for everything
POST /users/search
POST /users/list

# Good - Use appropriate methods
GET /users?search=term
GET /users
```

## Documentation

### OpenAPI Spec (Essential Sections)

```yaml
openapi: 3.0.3
info:
  title: My API
  version: 1.0.0

paths:
  /users:
    get:
      summary: List users
      parameters:
        - name: page
          in: query
          schema:
            type: integer
            default: 1
      responses:
        '200':
          description: Success
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/UserList'

components:
  schemas:
    User:
      type: object
      required: [id, email]
      properties:
        id:
          type: string
        email:
          type: string
          format: email
```

## Checklist

### Design Review

- [ ] Resources are nouns, plural, lowercase
- [ ] HTTP methods match CRUD operations
- [ ] Status codes are appropriate
- [ ] Error responses are consistent
- [ ] Pagination is implemented for lists
- [ ] Filtering/sorting uses query params
- [ ] Authentication is documented
- [ ] Rate limits are specified
- [ ] API versioning strategy defined
- [ ] OpenAPI spec is up to date
