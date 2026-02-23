---
name: fastapi
description: FastAPI patterns for building async APIs with validation, dependency injection, and OpenAPI. Use when building Python APIs.
---

# FastAPI Patterns

## Quick Reference

| Task | Command |
|------|---------|
| Dev server | `uvicorn main:app --reload` |
| Production | `uvicorn main:app --host 0.0.0.0 --port 8000` |
| Generate OpenAPI | Visit `/docs` or `/openapi.json` |
| Run tests | `pytest` |

| Decorator | HTTP Method |
|-----------|-------------|
| `@app.get()` | GET |
| `@app.post()` | POST |
| `@app.put()` | PUT |
| `@app.patch()` | PATCH |
| `@app.delete()` | DELETE |

## Project Structure

```
project/
├── app/
│   ├── __init__.py
│   ├── main.py              # FastAPI app instance
│   ├── config.py            # Settings management
│   ├── dependencies.py      # Shared dependencies
│   ├── models/              # SQLAlchemy/Pydantic models
│   │   ├── __init__.py
│   │   ├── user.py
│   │   └── item.py
│   ├── schemas/             # Pydantic schemas (API models)
│   │   ├── __init__.py
│   │   └── user.py
│   ├── routers/             # API routes
│   │   ├── __init__.py
│   │   ├── users.py
│   │   └── items.py
│   ├── services/            # Business logic
│   │   └── user_service.py
│   └── db/
│       ├── __init__.py
│       └── session.py
├── tests/
│   ├── conftest.py
│   └── test_users.py
├── alembic/                 # Migrations
├── pyproject.toml
└── .env
```

## Basic Application

```python
# app/main.py
from fastapi import FastAPI
from contextlib import asynccontextmanager
from app.routers import users, items
from app.db import engine

@asynccontextmanager
async def lifespan(app: FastAPI):
    # Startup
    await engine.connect()
    yield
    # Shutdown
    await engine.disconnect()

app = FastAPI(
    title="My API",
    version="1.0.0",
    lifespan=lifespan,
)

app.include_router(users.router, prefix="/users", tags=["users"])
app.include_router(items.router, prefix="/items", tags=["items"])

@app.get("/health")
async def health():
    return {"status": "ok"}
```

## Pydantic Schemas

```python
# app/schemas/user.py
from pydantic import BaseModel, EmailStr, Field
from datetime import datetime
from typing import Optional

class UserBase(BaseModel):
    email: EmailStr
    name: str = Field(..., min_length=1, max_length=100)

class UserCreate(UserBase):
    password: str = Field(..., min_length=8)

class UserUpdate(BaseModel):
    email: Optional[EmailStr] = None
    name: Optional[str] = Field(None, min_length=1, max_length=100)

class UserResponse(UserBase):
    id: int
    created_at: datetime
    is_active: bool

    model_config = {"from_attributes": True}

class UserListResponse(BaseModel):
    items: list[UserResponse]
    total: int
    page: int
    pages: int
```

## Route Handlers

```python
# app/routers/users.py
from fastapi import APIRouter, Depends, HTTPException, Query, status
from sqlalchemy.ext.asyncio import AsyncSession
from app.schemas.user import UserCreate, UserResponse, UserListResponse
from app.services.user_service import UserService
from app.dependencies import get_db, get_current_user

router = APIRouter()

@router.get("", response_model=UserListResponse)
async def list_users(
    page: int = Query(1, ge=1),
    limit: int = Query(10, ge=1, le=100),
    db: AsyncSession = Depends(get_db),
):
    """List all users with pagination."""
    service = UserService(db)
    users, total = await service.list_users(page=page, limit=limit)
    return UserListResponse(
        items=users,
        total=total,
        page=page,
        pages=(total + limit - 1) // limit,
    )

@router.get("/{user_id}", response_model=UserResponse)
async def get_user(
    user_id: int,
    db: AsyncSession = Depends(get_db),
):
    """Get user by ID."""
    service = UserService(db)
    user = await service.get_user(user_id)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="User not found",
        )
    return user

@router.post("", response_model=UserResponse, status_code=status.HTTP_201_CREATED)
async def create_user(
    user_in: UserCreate,
    db: AsyncSession = Depends(get_db),
):
    """Create a new user."""
    service = UserService(db)
    existing = await service.get_by_email(user_in.email)
    if existing:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Email already registered",
        )
    return await service.create_user(user_in)

@router.delete("/{user_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_user(
    user_id: int,
    db: AsyncSession = Depends(get_db),
    current_user: User = Depends(get_current_user),
):
    """Delete a user (admin only)."""
    if not current_user.is_admin:
        raise HTTPException(status_code=status.HTTP_403_FORBIDDEN)
    service = UserService(db)
    await service.delete_user(user_id)
```

## Dependency Injection

```python
# app/dependencies.py
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from sqlalchemy.ext.asyncio import AsyncSession
from app.db.session import async_session
from app.services.auth import verify_token

security = HTTPBearer()

async def get_db() -> AsyncSession:
    async with async_session() as session:
        try:
            yield session
        finally:
            await session.close()

async def get_current_user(
    credentials: HTTPAuthorizationCredentials = Depends(security),
    db: AsyncSession = Depends(get_db),
):
    token = credentials.credentials
    payload = verify_token(token)
    if not payload:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid token",
        )
    user = await db.get(User, payload["sub"])
    if not user:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED)
    return user

def require_role(role: str):
    async def role_checker(user: User = Depends(get_current_user)):
        if user.role != role:
            raise HTTPException(status_code=status.HTTP_403_FORBIDDEN)
        return user
    return role_checker

# Usage: @router.get("/admin", dependencies=[Depends(require_role("admin"))])
```

## Database Integration

### SQLAlchemy Async

```python
# app/db/session.py
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker
from app.config import settings

engine = create_async_engine(
    settings.database_url,
    echo=settings.debug,
    pool_size=5,
    max_overflow=10,
)

async_session = sessionmaker(
    engine,
    class_=AsyncSession,
    expire_on_commit=False,
)
```

### Service Layer

```python
# app/services/user_service.py
from sqlalchemy import select, func
from sqlalchemy.ext.asyncio import AsyncSession
from app.models.user import User
from app.schemas.user import UserCreate

class UserService:
    def __init__(self, db: AsyncSession):
        self.db = db

    async def get_user(self, user_id: int) -> User | None:
        return await self.db.get(User, user_id)

    async def get_by_email(self, email: str) -> User | None:
        result = await self.db.execute(
            select(User).where(User.email == email)
        )
        return result.scalar_one_or_none()

    async def list_users(
        self, page: int = 1, limit: int = 10
    ) -> tuple[list[User], int]:
        offset = (page - 1) * limit

        # Count query
        count_result = await self.db.execute(select(func.count(User.id)))
        total = count_result.scalar()

        # Data query
        result = await self.db.execute(
            select(User).offset(offset).limit(limit)
        )
        users = result.scalars().all()

        return users, total

    async def create_user(self, user_in: UserCreate) -> User:
        user = User(
            email=user_in.email,
            name=user_in.name,
            hashed_password=hash_password(user_in.password),
        )
        self.db.add(user)
        await self.db.commit()
        await self.db.refresh(user)
        return user
```

## Error Handling

```python
# app/main.py
from fastapi import Request
from fastapi.responses import JSONResponse

class AppException(Exception):
    def __init__(self, status_code: int, detail: str, code: str = None):
        self.status_code = status_code
        self.detail = detail
        self.code = code

@app.exception_handler(AppException)
async def app_exception_handler(request: Request, exc: AppException):
    return JSONResponse(
        status_code=exc.status_code,
        content={
            "detail": exc.detail,
            "code": exc.code,
        },
    )

@app.exception_handler(Exception)
async def global_exception_handler(request: Request, exc: Exception):
    return JSONResponse(
        status_code=500,
        content={"detail": "Internal server error"},
    )
```

## Background Tasks

```python
from fastapi import BackgroundTasks

async def send_email(email: str, message: str):
    # Async email sending
    await email_client.send(email, message)

@router.post("/users")
async def create_user(
    user_in: UserCreate,
    background_tasks: BackgroundTasks,
    db: AsyncSession = Depends(get_db),
):
    user = await UserService(db).create_user(user_in)
    background_tasks.add_task(send_email, user.email, "Welcome!")
    return user
```

## Testing

```python
# tests/conftest.py
import pytest
from httpx import AsyncClient
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker
from app.main import app
from app.dependencies import get_db
from app.models import Base

TEST_DATABASE_URL = "sqlite+aiosqlite:///./test.db"

@pytest.fixture
async def db_session():
    engine = create_async_engine(TEST_DATABASE_URL)
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    async_session = sessionmaker(engine, class_=AsyncSession)
    async with async_session() as session:
        yield session

    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.drop_all)

@pytest.fixture
async def client(db_session):
    def override_get_db():
        return db_session

    app.dependency_overrides[get_db] = override_get_db
    async with AsyncClient(app=app, base_url="http://test") as client:
        yield client
    app.dependency_overrides.clear()

# tests/test_users.py
import pytest

@pytest.mark.asyncio
async def test_create_user(client):
    response = await client.post("/users", json={
        "email": "test@example.com",
        "name": "Test User",
        "password": "password123",
    })
    assert response.status_code == 201
    data = response.json()
    assert data["email"] == "test@example.com"

@pytest.mark.asyncio
async def test_get_user_not_found(client):
    response = await client.get("/users/999")
    assert response.status_code == 404
```

## Configuration

```python
# app/config.py
from pydantic_settings import BaseSettings
from functools import lru_cache

class Settings(BaseSettings):
    database_url: str
    secret_key: str
    debug: bool = False
    cors_origins: list[str] = ["http://localhost:3000"]

    model_config = {"env_file": ".env"}

@lru_cache
def get_settings() -> Settings:
    return Settings()

settings = get_settings()
```

## Anti-patterns

### Avoid: Sync Operations in Async Routes

```python
# Bad: Blocking call in async route
@app.get("/users")
async def get_users():
    users = db.query(User).all()  # Sync SQLAlchemy
    return users

# Good: Use async operations
@app.get("/users")
async def get_users(db: AsyncSession = Depends(get_db)):
    result = await db.execute(select(User))
    return result.scalars().all()
```

### Avoid: Business Logic in Routes

```python
# Bad: Logic in route handler
@router.post("/orders")
async def create_order(order_in: OrderCreate, db: AsyncSession = Depends(get_db)):
    # Check inventory
    product = await db.get(Product, order_in.product_id)
    if product.stock < order_in.quantity:
        raise HTTPException(400, "Not enough stock")
    # Update stock
    product.stock -= order_in.quantity
    # Create order
    order = Order(**order_in.dict())
    # ... more logic

# Good: Delegate to service
@router.post("/orders")
async def create_order(order_in: OrderCreate, db: AsyncSession = Depends(get_db)):
    service = OrderService(db)
    return await service.create_order(order_in)
```
