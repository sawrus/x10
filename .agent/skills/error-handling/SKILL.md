---
agentic:
  generated_by: agentic
  source: "areas/software/frontend/skills/error-handling/SKILL.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---
# Skill: Frontend Error Handling

## When to load

When adding error boundaries, handling async errors, or building error UI states.

## Error Boundary

```tsx
class ErrorBoundary extends React.Component<
  { fallback: React.ComponentType<{ error: Error; reset: () => void }> },
  { error: Error | null }
> {
  state = { error: null };

  static getDerivedStateFromError(error: Error) {
    return { error };
  }

  componentDidCatch(error: Error, info: React.ErrorInfo) {
    logger.error('UI Error', { error: error.message, componentStack: info.componentStack });
  }

  render() {
    if (this.state.error) {
      const Fallback = this.props.fallback;
      return <Fallback error={this.state.error} reset={() => this.setState({ error: null })} />;
    }
    return this.props.children;
  }
}

// Usage:
<ErrorBoundary fallback={RouteErrorFallback}>
  <UserDashboard />
</ErrorBoundary>
```

## Error Classification

```ts
export class ApiError extends Error {
  constructor(public status: number, public body: unknown) {
    super(`API Error ${status}`);
  }
}

// In components:
if (error instanceof ApiError) {
  if (error.status === 401) return <LoginRedirect />;
  if (error.status === 403) return <ForbiddenMessage />;
  if (error.status === 404) return <NotFound />;
}
return <GenericError message={error.message} />;
```
