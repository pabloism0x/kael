---
name: go-concurrency
description: Go concurrency patterns with goroutines, channels, and sync primitives. Use when writing concurrent code.
---

# Go Concurrency Patterns

## Quick Reference

| Primitive | Use Case |
|-----------|----------|
| `go func()` | Start concurrent work |
| `chan T` | Communication between goroutines |
| `select` | Multiplex channels |
| `sync.Mutex` | Protect shared state |
| `sync.WaitGroup` | Wait for goroutines |
| `sync.Once` | One-time initialization |
| `context.Context` | Cancellation and deadlines |

## Channel Basics

### Unbuffered vs Buffered

```go
// Unbuffered: synchronous, blocks until received
ch := make(chan int)

// Buffered: async up to capacity
ch := make(chan int, 10)

// Send
ch <- value

// Receive
value := <-ch

// Close (sender only)
close(ch)

// Check if closed
value, ok := <-ch
if !ok {
    // channel closed
}
```

### Channel Direction

```go
// Send-only
func producer(out chan<- int) {
    out <- 42
}

// Receive-only
func consumer(in <-chan int) {
    val := <-in
}
```

## Common Patterns

### Worker Pool

```go
func WorkerPool(jobs <-chan Job, results chan<- Result, numWorkers int) {
    var wg sync.WaitGroup

    for i := 0; i < numWorkers; i++ {
        wg.Add(1)
        go func(workerID int) {
            defer wg.Done()
            for job := range jobs {
                result := process(job)
                results <- result
            }
        }(i)
    }

    wg.Wait()
    close(results)
}

// Usage
func main() {
    jobs := make(chan Job, 100)
    results := make(chan Result, 100)

    // Start workers
    go WorkerPool(jobs, results, 4)

    // Send jobs
    go func() {
        for _, job := range jobList {
            jobs <- job
        }
        close(jobs)
    }()

    // Collect results
    for result := range results {
        process(result)
    }
}
```

### Fan-Out, Fan-In

```go
// Fan-out: distribute work to multiple goroutines
func fanOut(input <-chan int, n int) []<-chan int {
    outputs := make([]<-chan int, n)
    for i := 0; i < n; i++ {
        outputs[i] = worker(input)
    }
    return outputs
}

func worker(input <-chan int) <-chan int {
    output := make(chan int)
    go func() {
        defer close(output)
        for v := range input {
            output <- process(v)
        }
    }()
    return output
}

// Fan-in: merge multiple channels into one
func fanIn(inputs ...<-chan int) <-chan int {
    output := make(chan int)
    var wg sync.WaitGroup

    for _, ch := range inputs {
        wg.Add(1)
        go func(c <-chan int) {
            defer wg.Done()
            for v := range c {
                output <- v
            }
        }(ch)
    }

    go func() {
        wg.Wait()
        close(output)
    }()

    return output
}
```

### Pipeline

```go
func generator(nums ...int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for _, n := range nums {
            out <- n
        }
    }()
    return out
}

func square(in <-chan int) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for n := range in {
            out <- n * n
        }
    }()
    return out
}

func filter(in <-chan int, predicate func(int) bool) <-chan int {
    out := make(chan int)
    go func() {
        defer close(out)
        for n := range in {
            if predicate(n) {
                out <- n
            }
        }
    }()
    return out
}

// Usage
func main() {
    // Pipeline: generate -> square -> filter
    c := generator(1, 2, 3, 4, 5)
    c = square(c)
    c = filter(c, func(n int) bool { return n > 10 })

    for v := range c {
        fmt.Println(v)
    }
}
```

## Context for Cancellation

### With Cancel

```go
func worker(ctx context.Context, id int, jobs <-chan Job) {
    for {
        select {
        case <-ctx.Done():
            fmt.Printf("Worker %d: cancelled\n", id)
            return
        case job, ok := <-jobs:
            if !ok {
                return
            }
            process(job)
        }
    }
}

func main() {
    ctx, cancel := context.WithCancel(context.Background())
    defer cancel()

    jobs := make(chan Job)

    for i := 0; i < 3; i++ {
        go worker(ctx, i, jobs)
    }

    // Send some jobs
    for j := 0; j < 10; j++ {
        jobs <- Job{ID: j}
    }

    // Cancel all workers
    cancel()
    time.Sleep(time.Second) // Wait for cleanup
}
```

### With Timeout

```go
func fetchWithTimeout(url string) ([]byte, error) {
    ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()

    req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
    if err != nil {
        return nil, err
    }

    resp, err := http.DefaultClient.Do(req)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()

    return io.ReadAll(resp.Body)
}
```

### Propagate Context

```go
func HandleRequest(ctx context.Context) error {
    // Pass context to all downstream operations
    user, err := fetchUser(ctx, userID)
    if err != nil {
        return err
    }

    orders, err := fetchOrders(ctx, user.ID)
    if err != nil {
        return err
    }

    return processOrders(ctx, orders)
}
```

## Select Statement

### Basic Select

```go
select {
case msg := <-ch1:
    fmt.Println("Received from ch1:", msg)
case msg := <-ch2:
    fmt.Println("Received from ch2:", msg)
case ch3 <- value:
    fmt.Println("Sent to ch3")
default:
    fmt.Println("No communication ready")
}
```

### Timeout with Select

```go
select {
case result := <-ch:
    return result, nil
case <-time.After(5 * time.Second):
    return nil, errors.New("timeout")
}
```

### Non-blocking Operations

```go
// Non-blocking receive
select {
case msg := <-ch:
    process(msg)
default:
    // Channel empty, do something else
}

// Non-blocking send
select {
case ch <- msg:
    // Sent
default:
    // Channel full
    log.Println("Dropping message, channel full")
}
```

## Sync Primitives

### WaitGroup

```go
func processAll(items []Item) {
    var wg sync.WaitGroup

    for _, item := range items {
        wg.Add(1)
        go func(i Item) {
            defer wg.Done()
            process(i)
        }(item)
    }

    wg.Wait() // Block until all done
}
```

### Mutex

```go
type Counter struct {
    mu    sync.Mutex
    value int
}

func (c *Counter) Increment() {
    c.mu.Lock()
    defer c.mu.Unlock()
    c.value++
}

func (c *Counter) Value() int {
    c.mu.Lock()
    defer c.mu.Unlock()
    return c.value
}

// RWMutex for read-heavy workloads
type Cache struct {
    mu    sync.RWMutex
    items map[string]any
}

func (c *Cache) Get(key string) (any, bool) {
    c.mu.RLock()
    defer c.mu.RUnlock()
    val, ok := c.items[key]
    return val, ok
}

func (c *Cache) Set(key string, value any) {
    c.mu.Lock()
    defer c.mu.Unlock()
    c.items[key] = value
}
```

### Once

```go
var (
    instance *Singleton
    once     sync.Once
)

func GetInstance() *Singleton {
    once.Do(func() {
        instance = &Singleton{}
        instance.init()
    })
    return instance
}
```

### Cond

```go
type Queue struct {
    mu    sync.Mutex
    cond  *sync.Cond
    items []int
}

func NewQueue() *Queue {
    q := &Queue{}
    q.cond = sync.NewCond(&q.mu)
    return q
}

func (q *Queue) Push(item int) {
    q.mu.Lock()
    q.items = append(q.items, item)
    q.cond.Signal() // Wake one waiter
    q.mu.Unlock()
}

func (q *Queue) Pop() int {
    q.mu.Lock()
    for len(q.items) == 0 {
        q.cond.Wait() // Release lock and wait
    }
    item := q.items[0]
    q.items = q.items[1:]
    q.mu.Unlock()
    return item
}
```

## errgroup for Error Handling

```go
import "golang.org/x/sync/errgroup"

func fetchAll(ctx context.Context, urls []string) ([]Response, error) {
    g, ctx := errgroup.WithContext(ctx)
    results := make([]Response, len(urls))

    for i, url := range urls {
        i, url := i, url // Capture loop variables
        g.Go(func() error {
            resp, err := fetch(ctx, url)
            if err != nil {
                return err
            }
            results[i] = resp
            return nil
        })
    }

    if err := g.Wait(); err != nil {
        return nil, err
    }
    return results, nil
}
```

## Semaphore Pattern

```go
type Semaphore struct {
    ch chan struct{}
}

func NewSemaphore(n int) *Semaphore {
    return &Semaphore{ch: make(chan struct{}, n)}
}

func (s *Semaphore) Acquire() {
    s.ch <- struct{}{}
}

func (s *Semaphore) Release() {
    <-s.ch
}

// Usage: limit concurrent operations
func processWithLimit(items []Item, limit int) {
    sem := NewSemaphore(limit)
    var wg sync.WaitGroup

    for _, item := range items {
        wg.Add(1)
        go func(i Item) {
            defer wg.Done()
            sem.Acquire()
            defer sem.Release()
            process(i)
        }(item)
    }

    wg.Wait()
}
```

## Anti-patterns

### Avoid: Goroutine Leak

```go
// Bad: goroutine never exits
func bad() {
    ch := make(chan int)
    go func() {
        val := <-ch // Blocks forever if ch is never closed/sent
        process(val)
    }()
    // ch is never used, goroutine leaks
}

// Good: use context for cancellation
func good(ctx context.Context) {
    ch := make(chan int)
    go func() {
        select {
        case val := <-ch:
            process(val)
        case <-ctx.Done():
            return
        }
    }()
}
```

### Avoid: Data Race

```go
// Bad: concurrent map access
var m = make(map[string]int)

go func() { m["a"] = 1 }()
go func() { _ = m["a"] }()

// Good: use sync.Map or mutex
var m sync.Map

go func() { m.Store("a", 1) }()
go func() { m.Load("a") }()
```

### Avoid: Closing Channel Multiple Times

```go
// Bad: panic on second close
close(ch)
close(ch) // panic!

// Good: use sync.Once
var closeOnce sync.Once
closeOnce.Do(func() { close(ch) })
```

## Testing Concurrent Code

```go
func TestConcurrent(t *testing.T) {
    // Use -race flag: go test -race ./...

    counter := NewCounter()
    var wg sync.WaitGroup

    for i := 0; i < 1000; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            counter.Increment()
        }()
    }

    wg.Wait()

    if counter.Value() != 1000 {
        t.Errorf("expected 1000, got %d", counter.Value())
    }
}
```
