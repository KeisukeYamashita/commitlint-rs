---
title: Type format
description: Check if the type format is valid
---

* Default:`ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
    type-format:
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

### Type must start with a capital letter

```yaml
rules:
    type-format:
        level: error
        format: ^[A-Z].*$
```
