# API Authentication

This page documents the authentication endpoints for the Part-DB API.

## Login

Authenticate a user and receive a JWT token.

### Request

```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "user",
  "password": "password"
}
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | Yes | User's username |
| `password` | string | Yes | User's password |

### Response

**Success (200 OK)**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": 1,
    "username": "user",
    "email": "user@example.com",
    "is_admin": false
  }
}
```

**Error (401 Unauthorized)**

```json
{
  "message": "Invalid credentials",
  "code": "UNAUTHORIZED"
}
```

### Example

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"password"}'
```

## Get Current User

Retrieve information about the currently authenticated user.

### Request

```http
GET /api/auth/me
Authorization: Bearer <token>
```

### Response

**Success (200 OK)**

```json
{
  "id": 1,
  "username": "user",
  "email": "user@example.com",
  "is_admin": false
}
```

**Error (401 Unauthorized)**

```json
{
  "message": "Authentication required",
  "code": "UNAUTHORIZED"
}
```

### Example

```bash
curl http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## Using the Token

After successful login, include the token in the `Authorization` header for all protected endpoints:

```http
GET /api/parts
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Token Structure

The JWT token contains:

| Claim | Description |
|-------|-------------|
| `sub` | User ID (subject) |
| `username` | Username |
| `is_admin` | Admin flag |
| `exp` | Expiration timestamp |
| `iat` | Issued at timestamp |

## Token Expiration

Tokens expire after the configured duration (default: 24 hours). After expiration, the client must re-authenticate.

To handle token expiration:

1. Check for 401 responses
2. Redirect to login page or show login modal
3. Re-authenticate and retry the request

## Logout

Logout is handled client-side by removing the stored token:

```javascript
localStorage.removeItem('token');
// Redirect to login
```

## Security Best Practices

1. **Store tokens securely** - Use `httpOnly` cookies in production
2. **Use HTTPS** - Always use HTTPS in production
3. **Short token expiry** - Use shorter expiry for sensitive operations
4. **Refresh tokens** - Implement refresh token rotation for long sessions
5. **Validate on every request** - The server validates tokens on each request
