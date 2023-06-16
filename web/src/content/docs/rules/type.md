---
title: Type
description: Allowlist for types
---

* Default: `ignore`

In this example, we assumed that you have a project with the following types:

```yaml
rules:
    type:
        level: error
        options:
            - feat
            - fix
```

## ❌ Bad

```console
chore(cli): fix typo
=> type chore is not allowed. Only ["feat", "fix"] are allowed
```

## ✅ Good

```console
fix(api): fix typo
```

## Example

```yaml
rules:
    scope:
        level: error
        options:
            - api
            - web
```
