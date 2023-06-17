---
title: Scope format
description: Check if the scope format is valid
---

* Default:`ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
    scope-format:
        level: error
        format: ^[A-Z].*$
```

## ❌ Bad

```console
feat(cli): added a new flag
```

## ✅ Good

```console
feat(cli): Added a new flag
```

## Example

### Scope must start with a capital letter

```yaml
rules:
    scope-format:
        level: error
        format: ^[A-Z].*$
```
