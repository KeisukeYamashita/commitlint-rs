---
title: Scope
description: Allowlist for scopes
---

* Default: `ignore`

In this example, we assumed that you have a project with the following scopes:

```yaml
rules:
  scope:
    level: error
    options:
      - api
      - web
```

## ❌ Bad

```console
chore(cli): fix typo
=> scope cli is not allowed. Only ["api", "web"] are allowed
```

## ✅ Good

```console
chore(api): fix typo
```

## Example

### Only allow scopes `api` and `web`

```yaml
rules:
  scope:
    level: error
    options:
      - api
      - web
```

### Disallow all scopes

```yaml
rules:
  scope:
    level: error
    options: [] # or [""]
  scope-empty:
    level: ignore
```
