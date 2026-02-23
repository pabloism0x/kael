---
name: nextjs
description: Next.js App Router patterns, server components, and data fetching. Use when building Next.js applications.
---

# Next.js Patterns

## Quick Reference

| Task | Command |
|------|---------|
| Dev server | `npm run dev` |
| Build | `npm run build` |
| Start production | `npm start` |
| Lint | `npm run lint` |

| File | Purpose |
|------|---------|
| `page.tsx` | Route UI |
| `layout.tsx` | Shared layout |
| `loading.tsx` | Loading UI |
| `error.tsx` | Error boundary |
| `not-found.tsx` | 404 page |
| `route.ts` | API endpoint |

## App Router Structure

```
app/
├── layout.tsx           # Root layout
├── page.tsx             # Home page (/)
├── globals.css
├── (auth)/              # Route group (no URL segment)
│   ├── login/
│   │   └── page.tsx     # /login
│   └── register/
│       └── page.tsx     # /register
├── dashboard/
│   ├── layout.tsx       # Dashboard layout
│   ├── page.tsx         # /dashboard
│   └── [id]/
│       └── page.tsx     # /dashboard/:id
├── api/
│   └── users/
│       └── route.ts     # /api/users
└── _components/         # Private folder (not routed)
    └── Header.tsx
```

## Server Components (Default)

### Data Fetching

```tsx
// app/users/page.tsx
// Server Component - no 'use client' directive
async function getUsers() {
  const res = await fetch('https://api.example.com/users', {
    next: { revalidate: 60 }, // ISR: revalidate every 60 seconds
  });
  if (!res.ok) throw new Error('Failed to fetch');
  return res.json();
}

export default async function UsersPage() {
  const users = await getUsers();

  return (
    <ul>
      {users.map((user: User) => (
        <li key={user.id}>{user.name}</li>
      ))}
    </ul>
  );
}
```

### Caching Strategies

```tsx
// No cache (always fresh)
fetch(url, { cache: 'no-store' });

// Cache forever (default for static)
fetch(url, { cache: 'force-cache' });

// Revalidate after N seconds (ISR)
fetch(url, { next: { revalidate: 60 } });

// Revalidate with tags
fetch(url, { next: { tags: ['users'] } });

// Manual revalidation
import { revalidateTag, revalidatePath } from 'next/cache';
revalidateTag('users');
revalidatePath('/dashboard');
```

## Client Components

```tsx
'use client';

import { useState } from 'react';

export function Counter() {
  const [count, setCount] = useState(0);

  return (
    <button onClick={() => setCount(c => c + 1)}>
      Count: {count}
    </button>
  );
}
```

### When to Use Client Components

| Use Client Component | Use Server Component |
|---------------------|---------------------|
| `useState`, `useEffect` | Data fetching |
| Event handlers (`onClick`) | Database access |
| Browser APIs | Sensitive data (API keys) |
| Third-party client libs | Large dependencies |

## Layouts

### Root Layout

```tsx
// app/layout.tsx
import { Inter } from 'next/font/google';
import './globals.css';

const inter = Inter({ subsets: ['latin'] });

export const metadata = {
  title: 'My App',
  description: 'App description',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Header />
        <main>{children}</main>
        <Footer />
      </body>
    </html>
  );
}
```

### Nested Layout

```tsx
// app/dashboard/layout.tsx
export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex">
      <Sidebar />
      <div className="flex-1">{children}</div>
    </div>
  );
}
```

## Loading & Error States

### Loading UI

```tsx
// app/dashboard/loading.tsx
export default function Loading() {
  return <div className="skeleton">Loading...</div>;
}
```

### Error Boundary

```tsx
// app/dashboard/error.tsx
'use client';

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <div>
      <h2>Something went wrong!</h2>
      <button onClick={() => reset()}>Try again</button>
    </div>
  );
}
```

### Not Found

```tsx
// app/not-found.tsx
import Link from 'next/link';

export default function NotFound() {
  return (
    <div>
      <h2>Page Not Found</h2>
      <Link href="/">Go home</Link>
    </div>
  );
}

// Trigger programmatically
import { notFound } from 'next/navigation';

async function getUser(id: string) {
  const user = await db.user.findUnique({ where: { id } });
  if (!user) notFound();
  return user;
}
```

## Server Actions

```tsx
// app/actions.ts
'use server';

import { revalidatePath } from 'next/cache';
import { redirect } from 'next/navigation';

export async function createUser(formData: FormData) {
  const name = formData.get('name') as string;
  const email = formData.get('email') as string;

  await db.user.create({ data: { name, email } });

  revalidatePath('/users');
  redirect('/users');
}

// In component
import { createUser } from './actions';

export function CreateUserForm() {
  return (
    <form action={createUser}>
      <input name="name" required />
      <input name="email" type="email" required />
      <button type="submit">Create</button>
    </form>
  );
}
```

### With useFormState

```tsx
'use client';

import { useFormState, useFormStatus } from 'react-dom';
import { createUser } from './actions';

function SubmitButton() {
  const { pending } = useFormStatus();
  return (
    <button type="submit" disabled={pending}>
      {pending ? 'Creating...' : 'Create'}
    </button>
  );
}

export function CreateUserForm() {
  const [state, formAction] = useFormState(createUser, null);

  return (
    <form action={formAction}>
      <input name="name" required />
      {state?.errors?.name && <p>{state.errors.name}</p>}
      <SubmitButton />
    </form>
  );
}
```

## API Routes

```tsx
// app/api/users/route.ts
import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const page = searchParams.get('page') ?? '1';

  const users = await db.user.findMany({
    skip: (parseInt(page) - 1) * 10,
    take: 10,
  });

  return NextResponse.json(users);
}

export async function POST(request: NextRequest) {
  const body = await request.json();

  const user = await db.user.create({ data: body });

  return NextResponse.json(user, { status: 201 });
}
```

### Dynamic Route Handlers

```tsx
// app/api/users/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server';

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  const user = await db.user.findUnique({
    where: { id: params.id },
  });

  if (!user) {
    return NextResponse.json({ error: 'Not found' }, { status: 404 });
  }

  return NextResponse.json(user);
}
```

## Middleware

```tsx
// middleware.ts (root level)
import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export function middleware(request: NextRequest) {
  // Check auth
  const token = request.cookies.get('token');

  if (!token && request.nextUrl.pathname.startsWith('/dashboard')) {
    return NextResponse.redirect(new URL('/login', request.url));
  }

  // Add headers
  const response = NextResponse.next();
  response.headers.set('x-custom-header', 'value');

  return response;
}

export const config = {
  matcher: ['/dashboard/:path*', '/api/:path*'],
};
```

## Metadata

### Static Metadata

```tsx
// app/page.tsx
export const metadata = {
  title: 'Home',
  description: 'Welcome to my app',
  openGraph: {
    title: 'Home',
    description: 'Welcome to my app',
    images: ['/og-image.png'],
  },
};
```

### Dynamic Metadata

```tsx
// app/posts/[slug]/page.tsx
import { Metadata } from 'next';

export async function generateMetadata({
  params,
}: {
  params: { slug: string };
}): Promise<Metadata> {
  const post = await getPost(params.slug);

  return {
    title: post.title,
    description: post.excerpt,
  };
}
```

## Static Generation

```tsx
// Generate static params at build time
export async function generateStaticParams() {
  const posts = await getPosts();

  return posts.map((post) => ({
    slug: post.slug,
  }));
}

// Force static/dynamic
export const dynamic = 'force-static'; // or 'force-dynamic'
export const revalidate = 60; // ISR interval
```

## Parallel & Intercepting Routes

### Parallel Routes

```
app/
├── layout.tsx
├── @modal/
│   └── login/
│       └── page.tsx
└── page.tsx
```

```tsx
// app/layout.tsx
export default function Layout({
  children,
  modal,
}: {
  children: React.ReactNode;
  modal: React.ReactNode;
}) {
  return (
    <>
      {children}
      {modal}
    </>
  );
}
```

### Intercepting Routes

```
app/
├── feed/
│   └── page.tsx
├── @modal/
│   └── (.)photo/[id]/    # Intercepts /photo/[id]
│       └── page.tsx
└── photo/[id]/
    └── page.tsx
```

## Anti-patterns

### Avoid: Client Components Wrapping Server Components

```tsx
// Bad: Server component inside client component loses benefits
'use client';
export function Wrapper() {
  return <ServerComponent />; // Now runs as client
}

// Good: Pass as children
'use client';
export function Wrapper({ children }) {
  return <div onClick={handleClick}>{children}</div>;
}

// Usage
<Wrapper>
  <ServerComponent />
</Wrapper>
```

### Avoid: Over-fetching in Layouts

```tsx
// Bad: Layout fetches data for all routes
// app/layout.tsx
export default async function Layout({ children }) {
  const user = await getUser(); // Runs on every navigation
  return <div>{children}</div>;
}

// Good: Fetch where needed or use React cache
import { cache } from 'react';
const getUser = cache(async () => {
  return db.user.findUnique({ where: { id: userId } });
});
```

## Environment Variables

```bash
# .env.local
DATABASE_URL=postgres://...
NEXT_PUBLIC_API_URL=https://api.example.com  # Exposed to browser
```

```tsx
// Server only
const dbUrl = process.env.DATABASE_URL;

// Client accessible (NEXT_PUBLIC_ prefix)
const apiUrl = process.env.NEXT_PUBLIC_API_URL;
```
