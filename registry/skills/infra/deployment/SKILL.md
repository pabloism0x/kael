---
name: deployment
description: Deployment strategies, rollout patterns, and release management. Use when planning or executing deployments.
---

# Deployment Patterns

## Quick Reference

| Strategy | Risk | Downtime | Rollback | Use Case |
|----------|------|----------|----------|----------|
| Recreate | High | Yes | Slow | Dev/Test |
| Rolling | Medium | No | Medium | Standard |
| Blue-Green | Low | No | Fast | Critical |
| Canary | Low | No | Fast | High traffic |
| Feature Flags | Low | No | Instant | Gradual rollout |

## Deployment Strategies

### Recreate (Big Bang)

```
Old Version ────────────■ STOP
                        │
                        ▼ Downtime
                        │
New Version             ■──────────►
```

```yaml
# Kubernetes
spec:
  strategy:
    type: Recreate
```

**Use when:**
- Development/staging environments
- Application can't run multiple versions
- Complete data migration required

### Rolling Update

```
Old Version ■■■■■■■■■■
New Version ────■■■■■■■■■■
                ↑ Gradual replacement
```

```yaml
# Kubernetes
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 25%        # Extra pods during update
      maxUnavailable: 25%  # Pods that can be down
```

**Parameters:**
- `maxSurge`: How many extra pods during update
- `maxUnavailable`: How many pods can be unavailable

### Blue-Green Deployment

```
           ┌─────────────┐
           │  Blue (v1)  │ ◄── Production traffic
           └─────────────┘
                 │
           ┌─────────────┐
           │ Green (v2)  │ ◄── Staging/Test
           └─────────────┘
                 │
           Switch traffic
                 │
                 ▼
           ┌─────────────┐
           │  Blue (v1)  │ ◄── Ready for rollback
           └─────────────┘
                 │
           ┌─────────────┐
           │ Green (v2)  │ ◄── Production traffic
           └─────────────┘
```

```yaml
# Kubernetes Services
apiVersion: v1
kind: Service
metadata:
  name: my-app
spec:
  selector:
    app: my-app
    version: green  # Switch to 'blue' to rollback
  ports:
    - port: 80
```

**Implementation:**
1. Deploy new version (green) alongside current (blue)
2. Test green environment
3. Switch traffic from blue to green
4. Keep blue for quick rollback

### Canary Deployment

```
Traffic Distribution:
├── 90% ──► Old Version (Stable)
└── 10% ──► New Version (Canary)

Gradual rollout:
10% → 25% → 50% → 75% → 100%
```

```yaml
# Istio VirtualService
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: my-app
spec:
  hosts:
    - my-app
  http:
    - route:
        - destination:
            host: my-app
            subset: stable
          weight: 90
        - destination:
            host: my-app
            subset: canary
          weight: 10
```

**Monitoring criteria:**
- Error rate < 1%
- Latency p99 < 500ms
- CPU/Memory within limits

### A/B Testing

```
User Segmentation:
├── Group A (50%) ──► Version A
└── Group B (50%) ──► Version B
```

```yaml
# Istio - Header-based routing
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
spec:
  http:
    - match:
        - headers:
            x-user-group:
              exact: beta
      route:
        - destination:
            host: my-app
            subset: v2
    - route:
        - destination:
            host: my-app
            subset: v1
```

## Feature Flags

### Implementation

```typescript
// Feature flag service
interface FeatureFlags {
  newCheckout: boolean;
  darkMode: boolean;
  betaFeatures: boolean;
}

async function getFeatureFlags(userId: string): Promise<FeatureFlags> {
  // From LaunchDarkly, Unleash, or custom service
  return await featureFlagService.getFlags(userId);
}

// Usage
const flags = await getFeatureFlags(user.id);
if (flags.newCheckout) {
  return <NewCheckoutFlow />;
}
return <LegacyCheckoutFlow />;
```

### Rollout Strategy

```
1. Internal users only (1%)
2. Beta users (5%)
3. Gradual rollout (10% → 25% → 50% → 100%)
4. Full release
5. Remove flag and old code
```

## Environment Promotion

### Pipeline Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│     Dev     │ ──► │   Staging   │ ──► │     UAT     │ ──► │ Production  │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
     │                    │                   │                    │
  Auto deploy        Auto deploy          Manual gate        Manual gate
  on merge           on tag              + smoke tests       + approval
```

### GitOps Workflow

```yaml
# ArgoCD Application
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: my-app-production
spec:
  project: default
  source:
    repoURL: https://github.com/org/k8s-manifests
    targetRevision: main
    path: overlays/production
  destination:
    server: https://kubernetes.default.svc
    namespace: production
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
```

## Rollback Strategies

### Kubernetes Rollback

```bash
# Check rollout history
kubectl rollout history deployment/my-app

# Rollback to previous version
kubectl rollout undo deployment/my-app

# Rollback to specific revision
kubectl rollout undo deployment/my-app --to-revision=2

# Monitor rollback
kubectl rollout status deployment/my-app
```

### Database Rollback

```sql
-- Forward migration
ALTER TABLE users ADD COLUMN phone VARCHAR(20);

-- Rollback migration (keep both ready)
ALTER TABLE users DROP COLUMN phone;
```

### Rollback Checklist

- [ ] Identify the issue and confirm rollback is needed
- [ ] Notify stakeholders
- [ ] Execute rollback (deployment)
- [ ] Verify application health
- [ ] Check database compatibility
- [ ] Monitor error rates and performance
- [ ] Post-mortem analysis

## Health Checks

### Kubernetes Probes

```yaml
spec:
  containers:
    - name: app
      livenessProbe:
        httpGet:
          path: /health/live
          port: 8080
        initialDelaySeconds: 10
        periodSeconds: 10
      readinessProbe:
        httpGet:
          path: /health/ready
          port: 8080
        initialDelaySeconds: 5
        periodSeconds: 5
      startupProbe:
        httpGet:
          path: /health/startup
          port: 8080
        failureThreshold: 30
        periodSeconds: 10
```

### Health Endpoint Design

```json
// GET /health/ready
{
  "status": "healthy",
  "checks": {
    "database": { "status": "up", "latency": "5ms" },
    "cache": { "status": "up", "latency": "1ms" },
    "queue": { "status": "up", "depth": 0 }
  },
  "version": "1.2.3",
  "uptime": "2d 5h 30m"
}
```

## Pre-deployment Checklist

### Code & Build

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Security scan completed
- [ ] Dependencies up to date
- [ ] Build artifacts created

### Configuration

- [ ] Environment variables set
- [ ] Secrets rotated if needed
- [ ] Feature flags configured
- [ ] Database migrations ready

### Infrastructure

- [ ] Sufficient resources allocated
- [ ] Auto-scaling configured
- [ ] Health checks defined
- [ ] Monitoring/alerts set up
- [ ] Rollback plan documented

### Communication

- [ ] Changelog prepared
- [ ] Stakeholders notified
- [ ] Support team briefed
- [ ] Status page ready

## Post-deployment Verification

### Smoke Tests

```bash
# Quick verification script
#!/bin/bash

BASE_URL="https://api.example.com"

# Health check
curl -sf "$BASE_URL/health" || exit 1

# Critical endpoints
curl -sf "$BASE_URL/api/v1/status" || exit 1
curl -sf -X POST "$BASE_URL/api/v1/echo" -d '{"test":true}' || exit 1

echo "Smoke tests passed"
```

### Metrics to Monitor

| Metric | Alert Threshold |
|--------|-----------------|
| Error rate | > 1% |
| Latency p99 | > 500ms |
| CPU usage | > 80% |
| Memory usage | > 85% |
| 5xx responses | > 0.1% |

## Deployment Automation

### GitHub Actions Example

```yaml
name: Deploy

on:
  push:
    tags: ['v*']

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@v4

      - name: Build and push image
        run: |
          docker build -t app:${{ github.sha }} .
          docker push registry/app:${{ github.sha }}

      - name: Deploy to Kubernetes
        run: |
          kubectl set image deployment/app app=registry/app:${{ github.sha }}
          kubectl rollout status deployment/app --timeout=5m

      - name: Run smoke tests
        run: ./scripts/smoke-test.sh

      - name: Rollback on failure
        if: failure()
        run: kubectl rollout undo deployment/app
```

## Anti-patterns

### Avoid: Deploy on Fridays

```
# Bad: Deploying before weekend
Friday 5pm → Deploy → Weekend issues → No one available

# Good: Deploy early in the week
Monday-Wednesday → Deploy → Monitor → Time to fix
```

### Avoid: Big Bang Releases

```
# Bad: 6 months of changes in one release
100 features + 50 bug fixes → High risk

# Good: Frequent small releases
1-3 features per release → Lower risk → Easier rollback
```

### Avoid: No Rollback Plan

```
# Bad: "We'll figure it out if something goes wrong"
# Good: Document and test rollback procedure before deploy
```

## Zero-Downtime Checklist

- [ ] Database changes are backward compatible
- [ ] API changes are backward compatible
- [ ] Health checks are properly configured
- [ ] Rolling update strategy configured
- [ ] Load balancer drains connections properly
- [ ] Sessions handled (sticky sessions or stateless)
- [ ] Cache invalidation planned
