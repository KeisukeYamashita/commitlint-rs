---
title: Type Empty
description: Check if the type exists
---

* Default: `error`

## ❌ Bad

```console
docs: fix typo
```

```console
(web): fix typo
```

## ✅ Good

```console
docs(web): fix typo
```

## Example

### Type must exist

```yaml
rules:
  type-empty:
    level: error
```
