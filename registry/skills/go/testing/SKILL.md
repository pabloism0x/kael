---
name: go-testing
description: Go testing patterns with table tests, mocks, and benchmarks. Use when writing or improving tests.
---

# Go Testing Patterns

## Quick Reference

| Command | Purpose |
|---------|---------|
| `go test ./...` | Run all tests |
| `go test -v` | Verbose output |
| `go test -run TestName` | Run specific test |
| `go test -race` | Race detector |
| `go test -cover` | Coverage report |
| `go test -bench=.` | Run benchmarks |
| `go test -count=1` | Disable test cache |

## Test File Structure

```go
// user_test.go
package user

import (
    "testing"
)

func TestUserCreate(t *testing.T) {
    // Arrange
    input := CreateUserInput{Name: "Test", Email: "test@example.com"}

    // Act
    user, err := CreateUser(input)

    // Assert
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if user.Name != input.Name {
        t.Errorf("got name %q, want %q", user.Name, input.Name)
    }
}
```

## Table-Driven Tests

```go
func TestAdd(t *testing.T) {
    tests := []struct {
        name string
        a, b int
        want int
    }{
        {"positive", 1, 2, 3},
        {"negative", -1, -2, -3},
        {"zero", 0, 0, 0},
        {"mixed", -1, 2, 1},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got := Add(tt.a, tt.b)
            if got != tt.want {
                t.Errorf("Add(%d, %d) = %d, want %d", tt.a, tt.b, got, tt.want)
            }
        })
    }
}
```

### With Error Cases

```go
func TestParseConfig(t *testing.T) {
    tests := []struct {
        name    string
        input   string
        want    *Config
        wantErr bool
    }{
        {
            name:  "valid config",
            input: `{"port": 8080}`,
            want:  &Config{Port: 8080},
        },
        {
            name:    "invalid json",
            input:   `{invalid}`,
            wantErr: true,
        },
        {
            name:    "empty input",
            input:   "",
            wantErr: true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            got, err := ParseConfig(tt.input)

            if tt.wantErr {
                if err == nil {
                    t.Fatal("expected error, got nil")
                }
                return
            }

            if err != nil {
                t.Fatalf("unexpected error: %v", err)
            }

            if got.Port != tt.want.Port {
                t.Errorf("got port %d, want %d", got.Port, tt.want.Port)
            }
        })
    }
}
```

## Subtests and Parallel

```go
func TestUserService(t *testing.T) {
    // Setup shared resources
    db := setupTestDB(t)

    t.Run("Create", func(t *testing.T) {
        t.Parallel()
        // test create
    })

    t.Run("Update", func(t *testing.T) {
        t.Parallel()
        // test update
    })

    t.Run("Delete", func(t *testing.T) {
        t.Parallel()
        // test delete
    })
}
```

## Test Helpers

```go
// Helper function
func assertEqual(t *testing.T, got, want any) {
    t.Helper() // Marks this as helper (better error location)
    if got != want {
        t.Errorf("got %v, want %v", got, want)
    }
}

// Setup/teardown helper
func setupTestDB(t *testing.T) *sql.DB {
    t.Helper()
    db, err := sql.Open("sqlite3", ":memory:")
    if err != nil {
        t.Fatalf("failed to open db: %v", err)
    }

    // Cleanup when test completes
    t.Cleanup(func() {
        db.Close()
    })

    return db
}
```

## Mocking

### Interface-Based Mocking

```go
// Define interface
type UserRepository interface {
    Get(ctx context.Context, id string) (*User, error)
    Create(ctx context.Context, user *User) error
}

// Mock implementation
type MockUserRepo struct {
    GetFunc    func(ctx context.Context, id string) (*User, error)
    CreateFunc func(ctx context.Context, user *User) error
}

func (m *MockUserRepo) Get(ctx context.Context, id string) (*User, error) {
    return m.GetFunc(ctx, id)
}

func (m *MockUserRepo) Create(ctx context.Context, user *User) error {
    return m.CreateFunc(ctx, user)
}

// Test with mock
func TestUserService_GetUser(t *testing.T) {
    mockRepo := &MockUserRepo{
        GetFunc: func(ctx context.Context, id string) (*User, error) {
            if id == "123" {
                return &User{ID: "123", Name: "Test"}, nil
            }
            return nil, ErrNotFound
        },
    }

    service := NewUserService(mockRepo)

    user, err := service.GetUser(context.Background(), "123")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if user.Name != "Test" {
        t.Errorf("got name %q, want %q", user.Name, "Test")
    }
}
```

### Using testify/mock

```go
import "github.com/stretchr/testify/mock"

type MockUserRepo struct {
    mock.Mock
}

func (m *MockUserRepo) Get(ctx context.Context, id string) (*User, error) {
    args := m.Called(ctx, id)
    if args.Get(0) == nil {
        return nil, args.Error(1)
    }
    return args.Get(0).(*User), args.Error(1)
}

func TestWithTestify(t *testing.T) {
    mockRepo := new(MockUserRepo)
    mockRepo.On("Get", mock.Anything, "123").Return(&User{ID: "123"}, nil)

    service := NewUserService(mockRepo)
    user, _ := service.GetUser(context.Background(), "123")

    assert.Equal(t, "123", user.ID)
    mockRepo.AssertExpectations(t)
}
```

## HTTP Testing

### Handler Testing

```go
func TestHealthHandler(t *testing.T) {
    req := httptest.NewRequest(http.MethodGet, "/health", nil)
    rec := httptest.NewRecorder()

    HealthHandler(rec, req)

    if rec.Code != http.StatusOK {
        t.Errorf("got status %d, want %d", rec.Code, http.StatusOK)
    }

    var resp map[string]string
    json.NewDecoder(rec.Body).Decode(&resp)

    if resp["status"] != "ok" {
        t.Errorf("got status %q, want %q", resp["status"], "ok")
    }
}
```

### Server Testing

```go
func TestAPIEndpoints(t *testing.T) {
    // Create test server
    srv := httptest.NewServer(NewRouter())
    defer srv.Close()

    // Make requests
    resp, err := http.Get(srv.URL + "/health")
    if err != nil {
        t.Fatalf("request failed: %v", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        t.Errorf("got status %d, want %d", resp.StatusCode, http.StatusOK)
    }
}
```

### Request with Body

```go
func TestCreateUser(t *testing.T) {
    body := `{"name": "Test", "email": "test@example.com"}`
    req := httptest.NewRequest(
        http.MethodPost,
        "/users",
        strings.NewReader(body),
    )
    req.Header.Set("Content-Type", "application/json")

    rec := httptest.NewRecorder()
    handler.CreateUser(rec, req)

    if rec.Code != http.StatusCreated {
        t.Errorf("got status %d, want %d", rec.Code, http.StatusCreated)
    }
}
```

## Benchmarks

```go
func BenchmarkSort(b *testing.B) {
    data := generateTestData(1000)

    b.ResetTimer() // Don't count setup time

    for i := 0; i < b.N; i++ {
        // Make copy to avoid sorting already sorted
        d := make([]int, len(data))
        copy(d, data)
        sort.Ints(d)
    }
}

// With different sizes
func BenchmarkSortSizes(b *testing.B) {
    sizes := []int{100, 1000, 10000}

    for _, size := range sizes {
        b.Run(fmt.Sprintf("size-%d", size), func(b *testing.B) {
            data := generateTestData(size)
            b.ResetTimer()

            for i := 0; i < b.N; i++ {
                d := make([]int, len(data))
                copy(d, data)
                sort.Ints(d)
            }
        })
    }
}

// Memory benchmark
func BenchmarkAlloc(b *testing.B) {
    b.ReportAllocs() // Report allocations

    for i := 0; i < b.N; i++ {
        _ = make([]byte, 1024)
    }
}
```

## Fuzzing

```go
func FuzzParseJSON(f *testing.F) {
    // Add seed corpus
    f.Add(`{"name": "test"}`)
    f.Add(`{}`)
    f.Add(`[]`)

    f.Fuzz(func(t *testing.T, data string) {
        // Should not panic
        _, _ = ParseJSON(data)
    })
}
```

## Test Fixtures

### testdata Directory

```
pkg/
├── parser.go
├── parser_test.go
└── testdata/
    ├── valid.json
    ├── invalid.json
    └── golden/
        └── expected_output.txt
```

```go
func TestParser(t *testing.T) {
    input, err := os.ReadFile("testdata/valid.json")
    if err != nil {
        t.Fatalf("failed to read test data: %v", err)
    }

    result, err := Parse(input)
    if err != nil {
        t.Fatalf("parse error: %v", err)
    }

    // Golden file comparison
    golden, _ := os.ReadFile("testdata/golden/expected_output.txt")
    if string(result) != string(golden) {
        t.Errorf("output mismatch")
    }
}
```

### Update Golden Files

```go
var update = flag.Bool("update", false, "update golden files")

func TestGolden(t *testing.T) {
    result := generateOutput()

    goldenFile := "testdata/golden/output.txt"

    if *update {
        os.WriteFile(goldenFile, []byte(result), 0644)
        return
    }

    expected, _ := os.ReadFile(goldenFile)
    if result != string(expected) {
        t.Errorf("output mismatch, run with -update to update golden file")
    }
}
```

## Integration Tests

```go
//go:build integration

package db_test

import (
    "testing"
)

func TestDatabaseIntegration(t *testing.T) {
    if testing.Short() {
        t.Skip("skipping integration test")
    }

    // Real database tests
}
```

```bash
# Run unit tests only
go test -short ./...

# Run all including integration
go test -tags=integration ./...
```

## Coverage

```bash
# Generate coverage
go test -coverprofile=coverage.out ./...

# View in browser
go tool cover -html=coverage.out

# Check coverage percentage
go test -cover ./...
```

## Anti-patterns

### Avoid: Testing Private Functions Directly

```go
// Bad: Export for testing
func TestPrivateHelper(t *testing.T) {
    result := privateHelper() // Won't compile
}

// Good: Test through public API
func TestPublicFunction(t *testing.T) {
    result := PublicFunction() // Uses privateHelper internally
    // Assert result
}
```

### Avoid: Flaky Time-Based Tests

```go
// Bad: Race condition with time
func TestTimeout(t *testing.T) {
    start := time.Now()
    doWork()
    if time.Since(start) > time.Second {
        t.Error("too slow")
    }
}

// Good: Use test doubles for time
type Clock interface {
    Now() time.Time
}

type MockClock struct {
    current time.Time
}

func (m *MockClock) Now() time.Time { return m.current }
```

### Avoid: Shared State Between Tests

```go
// Bad: Global state
var globalCounter int

func TestA(t *testing.T) {
    globalCounter++
}

func TestB(t *testing.T) {
    // Depends on TestA running first
}

// Good: Isolated state
func TestA(t *testing.T) {
    counter := 0
    counter++
}
```
