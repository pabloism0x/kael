---
name: react-patterns
description: React component patterns, hooks, and state management. Use when building React components or managing application state.
---

# React Patterns

## Quick Reference

| Pattern | Use Case |
|---------|----------|
| Functional Component | Default for all components |
| Custom Hook | Reusable stateful logic |
| Compound Component | Related components with shared state |
| Render Props | Dynamic rendering logic |
| HOC | Cross-cutting concerns (legacy) |
| Context | Global/shared state |
| Reducer | Complex state logic |

## Component Patterns

### Functional Component (Default)

```tsx
interface ButtonProps {
  variant?: 'primary' | 'secondary';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  onClick?: () => void;
  children: React.ReactNode;
}

export function Button({
  variant = 'primary',
  size = 'md',
  disabled = false,
  onClick,
  children,
}: ButtonProps) {
  return (
    <button
      className={`btn btn-${variant} btn-${size}`}
      disabled={disabled}
      onClick={onClick}
    >
      {children}
    </button>
  );
}
```

### Compound Component

```tsx
interface TabsContextValue {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

const TabsContext = createContext<TabsContextValue | null>(null);

function useTabs() {
  const context = useContext(TabsContext);
  if (!context) throw new Error('useTabs must be used within Tabs');
  return context;
}

function Tabs({ children, defaultTab }: { children: ReactNode; defaultTab: string }) {
  const [activeTab, setActiveTab] = useState(defaultTab);
  return (
    <TabsContext.Provider value={{ activeTab, setActiveTab }}>
      <div className="tabs">{children}</div>
    </TabsContext.Provider>
  );
}

function TabList({ children }: { children: ReactNode }) {
  return <div className="tab-list">{children}</div>;
}

function Tab({ id, children }: { id: string; children: ReactNode }) {
  const { activeTab, setActiveTab } = useTabs();
  return (
    <button
      className={activeTab === id ? 'active' : ''}
      onClick={() => setActiveTab(id)}
    >
      {children}
    </button>
  );
}

function TabPanel({ id, children }: { id: string; children: ReactNode }) {
  const { activeTab } = useTabs();
  if (activeTab !== id) return null;
  return <div className="tab-panel">{children}</div>;
}

Tabs.List = TabList;
Tabs.Tab = Tab;
Tabs.Panel = TabPanel;

// Usage
<Tabs defaultTab="tab1">
  <Tabs.List>
    <Tabs.Tab id="tab1">Tab 1</Tabs.Tab>
    <Tabs.Tab id="tab2">Tab 2</Tabs.Tab>
  </Tabs.List>
  <Tabs.Panel id="tab1">Content 1</Tabs.Panel>
  <Tabs.Panel id="tab2">Content 2</Tabs.Panel>
</Tabs>
```

## Hook Patterns

### Custom Hook Structure

```tsx
// hooks/useAsync.ts
interface AsyncState<T> {
  data: T | null;
  error: Error | null;
  isLoading: boolean;
}

export function useAsync<T>(asyncFn: () => Promise<T>, deps: unknown[] = []) {
  const [state, setState] = useState<AsyncState<T>>({
    data: null,
    error: null,
    isLoading: true,
  });

  useEffect(() => {
    let cancelled = false;
    setState(prev => ({ ...prev, isLoading: true }));

    asyncFn()
      .then(data => {
        if (!cancelled) setState({ data, error: null, isLoading: false });
      })
      .catch(error => {
        if (!cancelled) setState({ data: null, error, isLoading: false });
      });

    return () => { cancelled = true; };
  }, deps);

  return state;
}
```

### useReducer for Complex State

```tsx
type Action =
  | { type: 'FETCH_START' }
  | { type: 'FETCH_SUCCESS'; payload: Data[] }
  | { type: 'FETCH_ERROR'; error: Error }
  | { type: 'SELECT_ITEM'; id: string };

interface State {
  items: Data[];
  selectedId: string | null;
  isLoading: boolean;
  error: Error | null;
}

function reducer(state: State, action: Action): State {
  switch (action.type) {
    case 'FETCH_START':
      return { ...state, isLoading: true, error: null };
    case 'FETCH_SUCCESS':
      return { ...state, isLoading: false, items: action.payload };
    case 'FETCH_ERROR':
      return { ...state, isLoading: false, error: action.error };
    case 'SELECT_ITEM':
      return { ...state, selectedId: action.id };
    default:
      return state;
  }
}
```

### Event Handler Hook

```tsx
export function useEventCallback<T extends (...args: unknown[]) => unknown>(fn: T): T {
  const ref = useRef(fn);

  useLayoutEffect(() => {
    ref.current = fn;
  });

  return useCallback((...args: Parameters<T>) => {
    return ref.current(...args);
  }, []) as T;
}
```

## State Management

### Context + Reducer Pattern

```tsx
// store/UserContext.tsx
interface User {
  id: string;
  name: string;
  email: string;
}

type UserAction =
  | { type: 'SET_USER'; user: User }
  | { type: 'LOGOUT' };

interface UserState {
  user: User | null;
  isAuthenticated: boolean;
}

const UserContext = createContext<{
  state: UserState;
  dispatch: Dispatch<UserAction>;
} | null>(null);

function userReducer(state: UserState, action: UserAction): UserState {
  switch (action.type) {
    case 'SET_USER':
      return { user: action.user, isAuthenticated: true };
    case 'LOGOUT':
      return { user: null, isAuthenticated: false };
    default:
      return state;
  }
}

export function UserProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(userReducer, {
    user: null,
    isAuthenticated: false,
  });

  return (
    <UserContext.Provider value={{ state, dispatch }}>
      {children}
    </UserContext.Provider>
  );
}

export function useUser() {
  const context = useContext(UserContext);
  if (!context) throw new Error('useUser must be used within UserProvider');
  return context;
}
```

## Performance Patterns

### Memoization

```tsx
// Memoize expensive calculations
const sortedItems = useMemo(
  () => items.sort((a, b) => a.name.localeCompare(b.name)),
  [items]
);

// Memoize callbacks passed to children
const handleClick = useCallback((id: string) => {
  setSelectedId(id);
}, []);

// Memoize entire component
const MemoizedList = memo(function List({ items }: { items: Item[] }) {
  return items.map(item => <ListItem key={item.id} item={item} />);
});
```

### Virtualization for Large Lists

```tsx
// Use react-window or @tanstack/react-virtual
import { FixedSizeList } from 'react-window';

function VirtualList({ items }: { items: Item[] }) {
  return (
    <FixedSizeList
      height={400}
      width="100%"
      itemCount={items.length}
      itemSize={50}
    >
      {({ index, style }) => (
        <div style={style}>
          <ListItem item={items[index]} />
        </div>
      )}
    </FixedSizeList>
  );
}
```

## Anti-patterns

### Avoid: Props Drilling

```tsx
// Bad: Passing props through many levels
<App user={user}>
  <Layout user={user}>
    <Sidebar user={user}>
      <UserProfile user={user} />
    </Sidebar>
  </Layout>
</App>

// Good: Use Context
<UserProvider>
  <App>
    <Layout>
      <Sidebar>
        <UserProfile /> {/* Uses useUser() hook */}
      </Sidebar>
    </Layout>
  </App>
</UserProvider>
```

### Avoid: useEffect for Derived State

```tsx
// Bad: Syncing state with useEffect
const [items, setItems] = useState([]);
const [filteredItems, setFilteredItems] = useState([]);

useEffect(() => {
  setFilteredItems(items.filter(i => i.active));
}, [items]);

// Good: Compute during render
const [items, setItems] = useState([]);
const filteredItems = useMemo(
  () => items.filter(i => i.active),
  [items]
);
```

### Avoid: Object/Array as useEffect Dependency

```tsx
// Bad: New object reference every render
useEffect(() => {
  fetchData(options);
}, [options]); // options = { page: 1 } inline

// Good: Destructure primitives
useEffect(() => {
  fetchData({ page, limit });
}, [page, limit]);
```

## File Organization

```
src/
├── components/
│   ├── ui/                 # Generic UI components
│   │   ├── Button/
│   │   │   ├── Button.tsx
│   │   │   ├── Button.test.tsx
│   │   │   └── index.ts
│   │   └── Input/
│   ├── features/           # Feature-specific components
│   │   └── auth/
│   │       ├── LoginForm.tsx
│   │       └── useAuth.ts
│   └── layout/             # Layout components
│       ├── Header.tsx
│       └── Sidebar.tsx
├── hooks/                  # Shared custom hooks
│   ├── useAsync.ts
│   └── useLocalStorage.ts
├── context/                # Context providers
│   └── UserContext.tsx
└── utils/                  # Pure utility functions
    └── format.ts
```
