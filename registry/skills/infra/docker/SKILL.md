---
name: docker
description: Docker patterns for containerization, multi-stage builds, and compose. Use when containerizing applications.
---

# Docker Patterns

## Quick Reference

| Command | Purpose |
|---------|---------|
| `docker build -t app .` | Build image |
| `docker run -p 8080:80 app` | Run container |
| `docker compose up -d` | Start services |
| `docker compose down -v` | Stop and remove volumes |
| `docker logs -f container` | Follow logs |
| `docker exec -it container sh` | Shell into container |
| `docker system prune -a` | Clean unused resources |

## Dockerfile Best Practices

### Multi-stage Build (Node.js)

```dockerfile
# Build stage
FROM node:20-alpine AS builder
WORKDIR /app

# Install dependencies first (better caching)
COPY package*.json ./
RUN npm ci

# Copy source and build
COPY . .
RUN npm run build

# Production stage
FROM node:20-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production

# Create non-root user
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

# Copy only necessary files
COPY --from=builder --chown=nextjs:nodejs /app/dist ./dist
COPY --from=builder --chown=nextjs:nodejs /app/node_modules ./node_modules
COPY --from=builder --chown=nextjs:nodejs /app/package.json ./

USER nextjs
EXPOSE 3000

CMD ["node", "dist/main.js"]
```

### Multi-stage Build (Python)

```dockerfile
# Build stage
FROM python:3.12-slim AS builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create virtual environment
RUN python -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Production stage
FROM python:3.12-slim AS runner
WORKDIR /app

# Copy virtual environment
COPY --from=builder /opt/venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Create non-root user
RUN useradd --create-home --shell /bin/bash app
USER app

COPY --chown=app:app . .

EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```

### Multi-stage Build (Rust)

```dockerfile
# Build stage
FROM rust:1.75-alpine AS builder
WORKDIR /app

# Install musl for static linking
RUN apk add --no-cache musl-dev

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build actual application
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage (distroless)
FROM gcr.io/distroless/static-debian12
COPY --from=builder /app/target/release/app /app
EXPOSE 8080
ENTRYPOINT ["/app"]
```

### Multi-stage Build (Go)

```dockerfile
FROM golang:1.22-alpine AS builder
WORKDIR /app

# Download dependencies
COPY go.mod go.sum ./
RUN go mod download

# Build
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-w -s" -o /app/bin/server ./cmd/server

# Scratch image (minimal)
FROM scratch
COPY --from=builder /app/bin/server /server
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
EXPOSE 8080
ENTRYPOINT ["/server"]
```

## Docker Compose

### Development Setup

```yaml
# docker-compose.yml
services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - "3000:3000"
    volumes:
      - .:/app
      - /app/node_modules  # Anonymous volume for node_modules
    environment:
      - NODE_ENV=development
      - DATABASE_URL=postgres://user:pass@db:5432/myapp
    depends_on:
      db:
        condition: service_healthy

  db:
    image: postgres:16-alpine
    ports:
      - "5432:5432"
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: pass
      POSTGRES_DB: myapp
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user -d myapp"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

### Production Setup

```yaml
# docker-compose.prod.yml
services:
  app:
    image: myapp:latest
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: "0.5"
          memory: 512M
        reservations:
          cpus: "0.25"
          memory: 256M
      restart_policy:
        condition: on-failure
        max_attempts: 3
    environment:
      - NODE_ENV=production
    secrets:
      - db_password
    configs:
      - source: app_config
        target: /app/config.json

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./certs:/etc/nginx/certs:ro
    depends_on:
      - app

secrets:
  db_password:
    file: ./secrets/db_password.txt

configs:
  app_config:
    file: ./config/app.json
```

### Override Pattern

```yaml
# docker-compose.override.yml (auto-loaded in dev)
services:
  app:
    build:
      context: .
    volumes:
      - .:/app
    environment:
      - DEBUG=true
```

```bash
# Development (uses override automatically)
docker compose up

# Production (explicit file)
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## .dockerignore

```gitignore
# Git
.git
.gitignore

# Dependencies
node_modules
__pycache__
*.pyc
target/
vendor/

# Build outputs
dist
build
*.egg-info

# IDE
.idea
.vscode
*.swp

# Environment
.env
.env.*
!.env.example

# Tests
coverage
.pytest_cache
.nyc_output

# Docker
Dockerfile*
docker-compose*
.docker

# Documentation
*.md
docs/
```

## Health Checks

```dockerfile
# In Dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
```

```yaml
# In docker-compose.yml
services:
  app:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

## Build Arguments & Environment

```dockerfile
# Build-time variables
ARG NODE_VERSION=20
FROM node:${NODE_VERSION}-alpine

ARG BUILD_DATE
ARG GIT_SHA
LABEL org.opencontainers.image.created="${BUILD_DATE}"
LABEL org.opencontainers.image.revision="${GIT_SHA}"

# Runtime variables
ENV NODE_ENV=production
ENV PORT=3000
```

```bash
docker build \
  --build-arg NODE_VERSION=20 \
  --build-arg BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ) \
  --build-arg GIT_SHA=$(git rev-parse HEAD) \
  -t myapp:latest .
```

## Caching Strategies

### Layer Caching

```dockerfile
# Order matters - least changing first
COPY package.json package-lock.json ./  # Dependencies rarely change
RUN npm ci

COPY tsconfig.json ./                    # Config changes occasionally
COPY src ./src                           # Source changes frequently
RUN npm run build
```

### BuildKit Cache Mounts

```dockerfile
# syntax=docker/dockerfile:1
FROM node:20-alpine

# Cache npm packages
RUN --mount=type=cache,target=/root/.npm \
    npm ci

# Cache Go modules
RUN --mount=type=cache,target=/go/pkg/mod \
    go mod download

# Cache Rust dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release
```

## Security Best Practices

### Run as Non-root

```dockerfile
# Create user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Change ownership
COPY --chown=appuser:appgroup . .

# Switch user
USER appuser
```

### Read-only Filesystem

```yaml
services:
  app:
    read_only: true
    tmpfs:
      - /tmp
      - /var/run
```

### Scan for Vulnerabilities

```bash
# Scan image
docker scout cves myapp:latest

# Scan during build
docker build --sbom=true --provenance=true -t myapp .
```

## Anti-patterns

### Avoid: Running as Root

```dockerfile
# Bad
CMD ["node", "app.js"]

# Good
USER node
CMD ["node", "app.js"]
```

### Avoid: Storing Secrets in Image

```dockerfile
# Bad: Secret in environment
ENV DATABASE_PASSWORD=secret123

# Good: Use secrets at runtime
# docker run -e DATABASE_PASSWORD=$DB_PASS myapp
```

### Avoid: Using latest Tag

```dockerfile
# Bad
FROM node:latest

# Good: Pin version
FROM node:20.11-alpine
```

### Avoid: Large Images

```dockerfile
# Bad: Full OS image
FROM ubuntu:22.04

# Good: Minimal base
FROM node:20-alpine  # ~50MB vs ~1GB

# Better: Distroless
FROM gcr.io/distroless/nodejs20-debian12
```

## Debugging

```bash
# Inspect image layers
docker history myapp:latest

# Check image size
docker images myapp

# Dive into image (tool)
dive myapp:latest

# Debug running container
docker exec -it container_name sh

# Copy files from container
docker cp container:/app/logs ./logs
```
