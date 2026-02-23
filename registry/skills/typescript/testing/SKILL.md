---
name: typescript-testing
description: TypeScript testing patterns with Jest, Vitest, and React Testing Library. Use when writing or improving tests.
---

# TypeScript Testing

## Quick Reference

| Task | Jest | Vitest |
|------|------|--------|
| Run all | `npm test` | `npx vitest` |
| Watch mode | `npm test -- --watch` | `npx vitest --watch` |
| Coverage | `npm test -- --coverage` | `npx vitest --coverage` |
| Single file | `npm test -- path/to/file` | `npx vitest path/to/file` |
| Pattern match | `npm test -- -t "pattern"` | `npx vitest -t "pattern"` |

## Test Structure

### Basic Test File

```typescript
// sum.test.ts
import { describe, it, expect, beforeEach, afterEach } from 'vitest'; // or jest globals

describe('sum', () => {
  describe('with valid inputs', () => {
    it('should add two positive numbers', () => {
      expect(sum(1, 2)).toBe(3);
    });

    it('should handle negative numbers', () => {
      expect(sum(-1, -2)).toBe(-3);
    });
  });

  describe('with invalid inputs', () => {
    it('should throw for non-numbers', () => {
      expect(() => sum('a' as any, 1)).toThrow(TypeError);
    });
  });
});
```

### Async Testing

```typescript
describe('fetchUser', () => {
  it('should fetch user data', async () => {
    const user = await fetchUser('123');
    expect(user).toEqual({
      id: '123',
      name: 'John',
    });
  });

  it('should reject for invalid id', async () => {
    await expect(fetchUser('invalid')).rejects.toThrow('User not found');
  });
});
```

## Mocking Patterns

### Module Mocking

```typescript
// Mock entire module
vi.mock('./api', () => ({
  fetchUser: vi.fn(),
  updateUser: vi.fn(),
}));

// Mock with implementation
vi.mock('./config', () => ({
  getConfig: vi.fn(() => ({ apiUrl: 'http://test.com' })),
}));

// Partial mock (keep other exports)
vi.mock('./utils', async () => {
  const actual = await vi.importActual('./utils');
  return {
    ...actual,
    formatDate: vi.fn(() => '2024-01-01'),
  };
});
```

### Function Mocking

```typescript
describe('UserService', () => {
  const mockFetch = vi.fn();

  beforeEach(() => {
    mockFetch.mockReset();
  });

  it('should call API with correct params', async () => {
    mockFetch.mockResolvedValueOnce({ id: '1', name: 'Test' });

    const service = new UserService(mockFetch);
    await service.getUser('1');

    expect(mockFetch).toHaveBeenCalledWith('/users/1');
    expect(mockFetch).toHaveBeenCalledTimes(1);
  });

  it('should handle API errors', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network error'));

    const service = new UserService(mockFetch);
    await expect(service.getUser('1')).rejects.toThrow('Network error');
  });
});
```

### Spy on Methods

```typescript
it('should log errors', () => {
  const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

  processData(invalidData);

  expect(consoleSpy).toHaveBeenCalledWith('Invalid data:', expect.any(Error));
  consoleSpy.mockRestore();
});
```

## React Testing Library

### Component Testing

```tsx
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

describe('LoginForm', () => {
  it('should render login form', () => {
    render(<LoginForm />);

    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /sign in/i })).toBeInTheDocument();
  });

  it('should submit form with valid data', async () => {
    const user = userEvent.setup();
    const onSubmit = vi.fn();

    render(<LoginForm onSubmit={onSubmit} />);

    await user.type(screen.getByLabelText(/email/i), 'test@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    expect(onSubmit).toHaveBeenCalledWith({
      email: 'test@example.com',
      password: 'password123',
    });
  });

  it('should show validation errors', async () => {
    const user = userEvent.setup();
    render(<LoginForm />);

    await user.click(screen.getByRole('button', { name: /sign in/i }));

    expect(await screen.findByText(/email is required/i)).toBeInTheDocument();
  });
});
```

### Testing Hooks

```tsx
import { renderHook, act, waitFor } from '@testing-library/react';

describe('useCounter', () => {
  it('should increment counter', () => {
    const { result } = renderHook(() => useCounter(0));

    act(() => {
      result.current.increment();
    });

    expect(result.current.count).toBe(1);
  });
});

describe('useAsync', () => {
  it('should handle async data', async () => {
    const mockFetch = vi.fn().mockResolvedValue({ data: 'test' });

    const { result } = renderHook(() => useAsync(mockFetch));

    expect(result.current.isLoading).toBe(true);

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.data).toEqual({ data: 'test' });
  });
});
```

### Testing with Context

```tsx
function renderWithProviders(ui: ReactElement, options?: RenderOptions) {
  function Wrapper({ children }: { children: ReactNode }) {
    return (
      <QueryClientProvider client={new QueryClient()}>
        <ThemeProvider>
          <UserProvider>
            {children}
          </UserProvider>
        </ThemeProvider>
      </QueryClientProvider>
    );
  }

  return render(ui, { wrapper: Wrapper, ...options });
}

describe('Dashboard', () => {
  it('should show user name', () => {
    renderWithProviders(<Dashboard />);
    expect(screen.getByText(/welcome, john/i)).toBeInTheDocument();
  });
});
```

## API Testing

### MSW (Mock Service Worker)

```typescript
// mocks/handlers.ts
import { rest } from 'msw';

export const handlers = [
  rest.get('/api/users/:id', (req, res, ctx) => {
    const { id } = req.params;
    return res(
      ctx.json({ id, name: 'Test User', email: 'test@example.com' })
    );
  }),

  rest.post('/api/users', async (req, res, ctx) => {
    const body = await req.json();
    return res(ctx.status(201), ctx.json({ id: '123', ...body }));
  }),

  rest.get('/api/error', (req, res, ctx) => {
    return res(ctx.status(500), ctx.json({ message: 'Server error' }));
  }),
];

// mocks/server.ts
import { setupServer } from 'msw/node';
import { handlers } from './handlers';

export const server = setupServer(...handlers);

// setup.ts
beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());
```

### Testing API Errors

```typescript
it('should handle server error', async () => {
  server.use(
    rest.get('/api/users/:id', (req, res, ctx) => {
      return res(ctx.status(500));
    })
  );

  render(<UserProfile userId="123" />);

  expect(await screen.findByText(/error loading user/i)).toBeInTheDocument();
});
```

## Snapshot Testing

```typescript
describe('Button', () => {
  it('should match snapshot', () => {
    const { container } = render(
      <Button variant="primary" size="lg">Click me</Button>
    );
    expect(container).toMatchSnapshot();
  });

  // Inline snapshot (preferred for small outputs)
  it('should render correct classes', () => {
    const { container } = render(<Button variant="primary" />);
    expect(container.firstChild).toMatchInlineSnapshot(`
      <button class="btn btn-primary">
        Click me
      </button>
    `);
  });
});
```

## Test Utilities

### Custom Matchers

```typescript
// test/matchers.ts
expect.extend({
  toBeWithinRange(received: number, floor: number, ceiling: number) {
    const pass = received >= floor && received <= ceiling;
    return {
      pass,
      message: () =>
        `expected ${received} ${pass ? 'not ' : ''}to be within range ${floor} - ${ceiling}`,
    };
  },
});

// Usage
expect(100).toBeWithinRange(90, 110);
```

### Test Data Factories

```typescript
// test/factories.ts
import { faker } from '@faker-js/faker';

export function createUser(overrides: Partial<User> = {}): User {
  return {
    id: faker.string.uuid(),
    name: faker.person.fullName(),
    email: faker.internet.email(),
    createdAt: faker.date.past(),
    ...overrides,
  };
}

export function createUsers(count: number): User[] {
  return Array.from({ length: count }, () => createUser());
}

// Usage
const user = createUser({ name: 'Custom Name' });
const users = createUsers(10);
```

## Anti-patterns

### Avoid: Testing Implementation Details

```typescript
// Bad: Testing internal state
it('should set loading to true', () => {
  const { result } = renderHook(() => useData());
  expect(result.current.state.loading).toBe(true); // Internal state
});

// Good: Test observable behavior
it('should show loading indicator', () => {
  render(<DataComponent />);
  expect(screen.getByRole('progressbar')).toBeInTheDocument();
});
```

### Avoid: Hardcoded Waits

```typescript
// Bad: Fixed timeout
await new Promise(resolve => setTimeout(resolve, 1000));
expect(screen.getByText('Loaded')).toBeInTheDocument();

// Good: Wait for condition
await waitFor(() => {
  expect(screen.getByText('Loaded')).toBeInTheDocument();
});
```

### Avoid: Multiple Assertions Without Context

```typescript
// Bad: Many unrelated assertions
it('should work', () => {
  expect(a).toBe(1);
  expect(b).toBe(2);
  expect(c).toBe(3);
  expect(d).toBe(4);
});

// Good: Focused tests with clear intent
it('should calculate total correctly', () => {
  expect(calculateTotal(items)).toBe(100);
});

it('should apply discount', () => {
  expect(applyDiscount(100, 0.1)).toBe(90);
});
```

## Configuration

### Vitest Config

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['./test/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      exclude: ['node_modules', 'test/**'],
    },
  },
});
```

### Jest Config

```javascript
// jest.config.js
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/test/setup.ts'],
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
  collectCoverageFrom: [
    'src/**/*.{ts,tsx}',
    '!src/**/*.d.ts',
  ],
};
```
