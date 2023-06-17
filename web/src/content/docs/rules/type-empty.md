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
docs(): fix typo
```

## ✅ Good

```console
docs(scope): fix typo
```

## Example

### Type must exist

```yaml
rules:
    type-empty:
        level: error
```
