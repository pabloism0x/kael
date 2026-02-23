---
name: go-api-patterns
description: Go API patterns with net/http, Chi, and Echo. Use when building REST APIs in Go.
---

# Go API Patterns

## Quick Reference

| Task | Command |
|------|---------|
| Run server | `go run ./cmd/server` |
| Test | `go test ./...` |
| Build | `go build -o bin/server ./cmd/server` |
| Generate | `go generate ./...` |

| Framework | Use Case |
|-----------|----------|
| `net/http` | Standard library, minimal deps |
| `chi` | Lightweight router with middleware |
| `echo` | Full-featured with validation |
| `gin` | High performance with middleware |

## Project Structure

```
project/
├── cmd/
│   └── server/
│       └── main.go           # Entry point
├── internal/
│   ├── api/
│   │   ├── handler/          # HTTP handlers
│   │   │   ├── user.go
│   │   │   └── health.go
│   │   ├── middleware/       # HTTP middleware
│   │   │   ├── auth.go
│   │   │   └── logging.go
│   │   └── router.go         # Route definitions
│   ├── domain/               # Business logic
│   │   ├── user.go
│   │   └── user_service.go
│   ├── repository/           # Data access
│   │   └── user_repo.go
│   └── config/
│       └── config.go
├── pkg/                      # Public packages
│   └── response/
│       └── json.go
├── go.mod
└── go.sum
```

## Standard Library (net/http)

### Basic Server

```go
// cmd/server/main.go
package main

import (
    "context"
    "log"
    "net/http"
    "os"
    "os/signal"
    "syscall"
    "time"

    "myapp/internal/api"
    "myapp/internal/config"
)

func main() {
    cfg := config.Load()

    router := api.NewRouter(cfg)

    srv := &http.Server{
        Addr:         ":" + cfg.Port,
        Handler:      router,
        ReadTimeout:  15 * time.Second,
        WriteTimeout: 15 * time.Second,
        IdleTimeout:  60 * time.Second,
    }

    // Graceful shutdown
    go func() {
        log.Printf("Server starting on :%s", cfg.Port)
        if err := srv.ListenAndServe(); err != http.ErrServerClosed {
            log.Fatalf("Server error: %v", err)
        }
    }()

    quit := make(chan os.Signal, 1)
    signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
    <-quit

    log.Println("Shutting down...")
    ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
    defer cancel()

    if err := srv.Shutdown(ctx); err != nil {
        log.Fatalf("Shutdown error: %v", err)
    }
}
```

### Router with Middleware

```go
// internal/api/router.go
package api

import (
    "net/http"

    "myapp/internal/api/handler"
    "myapp/internal/api/middleware"
)

func NewRouter(cfg *config.Config) http.Handler {
    mux := http.NewServeMux()

    // Handlers
    userHandler := handler.NewUserHandler(cfg.DB)

    // Routes
    mux.HandleFunc("GET /health", handler.Health)
    mux.HandleFunc("GET /users", userHandler.List)
    mux.HandleFunc("GET /users/{id}", userHandler.Get)
    mux.HandleFunc("POST /users", userHandler.Create)
    mux.HandleFunc("PUT /users/{id}", userHandler.Update)
    mux.HandleFunc("DELETE /users/{id}", userHandler.Delete)

    // Apply middleware (outermost first)
    var h http.Handler = mux
    h = middleware.Logger(h)
    h = middleware.Recover(h)
    h = middleware.CORS(h)

    return h
}
```

### Handler Pattern

```go
// internal/api/handler/user.go
package handler

import (
    "encoding/json"
    "net/http"

    "myapp/internal/domain"
    "myapp/pkg/response"
)

type UserHandler struct {
    service *domain.UserService
}

func NewUserHandler(db *sql.DB) *UserHandler {
    return &UserHandler{
        service: domain.NewUserService(db),
    }
}

func (h *UserHandler) List(w http.ResponseWriter, r *http.Request) {
    users, err := h.service.List(r.Context())
    if err != nil {
        response.Error(w, http.StatusInternalServerError, "Failed to fetch users")
        return
    }
    response.JSON(w, http.StatusOK, users)
}

func (h *UserHandler) Get(w http.ResponseWriter, r *http.Request) {
    id := r.PathValue("id")

    user, err := h.service.Get(r.Context(), id)
    if err != nil {
        if errors.Is(err, domain.ErrNotFound) {
            response.Error(w, http.StatusNotFound, "User not found")
            return
        }
        response.Error(w, http.StatusInternalServerError, "Failed to fetch user")
        return
    }
    response.JSON(w, http.StatusOK, user)
}

func (h *UserHandler) Create(w http.ResponseWriter, r *http.Request) {
    var input domain.CreateUserInput
    if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
        response.Error(w, http.StatusBadRequest, "Invalid request body")
        return
    }

    if err := input.Validate(); err != nil {
        response.Error(w, http.StatusBadRequest, err.Error())
        return
    }

    user, err := h.service.Create(r.Context(), input)
    if err != nil {
        response.Error(w, http.StatusInternalServerError, "Failed to create user")
        return
    }
    response.JSON(w, http.StatusCreated, user)
}
```

### Response Helper

```go
// pkg/response/json.go
package response

import (
    "encoding/json"
    "net/http"
)

type ErrorResponse struct {
    Error   string `json:"error"`
    Code    int    `json:"code"`
    Details any    `json:"details,omitempty"`
}

func JSON(w http.ResponseWriter, status int, data any) {
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(status)
    json.NewEncoder(w).Encode(data)
}

func Error(w http.ResponseWriter, status int, message string) {
    JSON(w, status, ErrorResponse{
        Error: message,
        Code:  status,
    })
}
```

## Middleware Patterns

### Logging Middleware

```go
// internal/api/middleware/logging.go
package middleware

import (
    "log/slog"
    "net/http"
    "time"
)

type responseWriter struct {
    http.ResponseWriter
    status int
    size   int
}

func (rw *responseWriter) WriteHeader(status int) {
    rw.status = status
    rw.ResponseWriter.WriteHeader(status)
}

func (rw *responseWriter) Write(b []byte) (int, error) {
    size, err := rw.ResponseWriter.Write(b)
    rw.size += size
    return size, err
}

func Logger(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        start := time.Now()

        rw := &responseWriter{ResponseWriter: w, status: http.StatusOK}
        next.ServeHTTP(rw, r)

        slog.Info("request",
            "method", r.Method,
            "path", r.URL.Path,
            "status", rw.status,
            "size", rw.size,
            "duration", time.Since(start),
            "ip", r.RemoteAddr,
        )
    })
}
```

### Auth Middleware

```go
// internal/api/middleware/auth.go
package middleware

import (
    "context"
    "net/http"
    "strings"
)

type contextKey string

const UserKey contextKey = "user"

func Auth(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        header := r.Header.Get("Authorization")
        if header == "" {
            http.Error(w, "Unauthorized", http.StatusUnauthorized)
            return
        }

        token := strings.TrimPrefix(header, "Bearer ")
        user, err := validateToken(token)
        if err != nil {
            http.Error(w, "Invalid token", http.StatusUnauthorized)
            return
        }

        ctx := context.WithValue(r.Context(), UserKey, user)
        next.ServeHTTP(w, r.WithContext(ctx))
    })
}

// Get user from context
func UserFromContext(ctx context.Context) (*User, bool) {
    user, ok := ctx.Value(UserKey).(*User)
    return user, ok
}
```

### Recovery Middleware

```go
func Recover(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        defer func() {
            if err := recover(); err != nil {
                slog.Error("panic recovered",
                    "error", err,
                    "stack", string(debug.Stack()),
                )
                http.Error(w, "Internal Server Error", http.StatusInternalServerError)
            }
        }()
        next.ServeHTTP(w, r)
    })
}
```

## Chi Router

```go
import "github.com/go-chi/chi/v5"
import "github.com/go-chi/chi/v5/middleware"

func NewRouter() http.Handler {
    r := chi.NewRouter()

    // Middleware stack
    r.Use(middleware.RequestID)
    r.Use(middleware.RealIP)
    r.Use(middleware.Logger)
    r.Use(middleware.Recoverer)
    r.Use(middleware.Timeout(60 * time.Second))

    // Routes
    r.Get("/health", healthHandler)

    // User routes with auth
    r.Route("/users", func(r chi.Router) {
        r.Use(authMiddleware)

        r.Get("/", listUsers)
        r.Post("/", createUser)

        r.Route("/{userID}", func(r chi.Router) {
            r.Use(userCtx)
            r.Get("/", getUser)
            r.Put("/", updateUser)
            r.Delete("/", deleteUser)
        })
    })

    return r
}

func userCtx(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        userID := chi.URLParam(r, "userID")
        user, err := getUserByID(userID)
        if err != nil {
            http.Error(w, "User not found", http.StatusNotFound)
            return
        }
        ctx := context.WithValue(r.Context(), "user", user)
        next.ServeHTTP(w, r.WithContext(ctx))
    })
}
```

## Validation

```go
// Using go-playground/validator
import "github.com/go-playground/validator/v10"

var validate = validator.New()

type CreateUserInput struct {
    Email    string `json:"email" validate:"required,email"`
    Name     string `json:"name" validate:"required,min=2,max=100"`
    Password string `json:"password" validate:"required,min=8"`
}

func (i *CreateUserInput) Validate() error {
    return validate.Struct(i)
}

// Custom validation error response
func ValidationErrors(err error) map[string]string {
    errors := make(map[string]string)

    for _, e := range err.(validator.ValidationErrors) {
        field := strings.ToLower(e.Field())
        switch e.Tag() {
        case "required":
            errors[field] = "This field is required"
        case "email":
            errors[field] = "Invalid email format"
        case "min":
            errors[field] = fmt.Sprintf("Minimum length is %s", e.Param())
        default:
            errors[field] = "Invalid value"
        }
    }

    return errors
}
```

## Error Handling

```go
// internal/domain/errors.go
package domain

import "errors"

var (
    ErrNotFound      = errors.New("not found")
    ErrAlreadyExists = errors.New("already exists")
    ErrInvalidInput  = errors.New("invalid input")
    ErrUnauthorized  = errors.New("unauthorized")
)

// Custom error with code
type AppError struct {
    Code    int
    Message string
    Err     error
}

func (e *AppError) Error() string {
    if e.Err != nil {
        return fmt.Sprintf("%s: %v", e.Message, e.Err)
    }
    return e.Message
}

func (e *AppError) Unwrap() error {
    return e.Err
}

// Error handler middleware
func ErrorHandler(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        next.ServeHTTP(w, r)
    })
}
```

## Testing

```go
// internal/api/handler/user_test.go
package handler_test

import (
    "bytes"
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "testing"

    "myapp/internal/api/handler"
)

func TestUserHandler_Create(t *testing.T) {
    h := handler.NewUserHandler(mockDB)

    tests := []struct {
        name       string
        body       map[string]any
        wantStatus int
    }{
        {
            name:       "valid input",
            body:       map[string]any{"email": "test@example.com", "name": "Test"},
            wantStatus: http.StatusCreated,
        },
        {
            name:       "invalid email",
            body:       map[string]any{"email": "invalid", "name": "Test"},
            wantStatus: http.StatusBadRequest,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            body, _ := json.Marshal(tt.body)
            req := httptest.NewRequest(http.MethodPost, "/users", bytes.NewReader(body))
            req.Header.Set("Content-Type", "application/json")

            rec := httptest.NewRecorder()
            h.Create(rec, req)

            if rec.Code != tt.wantStatus {
                t.Errorf("got status %d, want %d", rec.Code, tt.wantStatus)
            }
        })
    }
}
```

## Anti-patterns

### Avoid: Blocking Operations Without Context

```go
// Bad
func (h *Handler) Get(w http.ResponseWriter, r *http.Request) {
    user, err := h.db.GetUser(id) // No context
}

// Good
func (h *Handler) Get(w http.ResponseWriter, r *http.Request) {
    user, err := h.db.GetUser(r.Context(), id) // With context
}
```

### Avoid: Gorilla Mux Pattern (Deprecated)

```go
// Old style (gorilla/mux)
r.HandleFunc("/users/{id}", handler).Methods("GET")
id := mux.Vars(r)["id"]

// New style (Go 1.22+)
mux.HandleFunc("GET /users/{id}", handler)
id := r.PathValue("id")
```

### Avoid: No Timeout

```go
// Bad
srv := &http.Server{Addr: ":8080", Handler: h}

// Good
srv := &http.Server{
    Addr:         ":8080",
    Handler:      h,
    ReadTimeout:  15 * time.Second,
    WriteTimeout: 15 * time.Second,
}
```
