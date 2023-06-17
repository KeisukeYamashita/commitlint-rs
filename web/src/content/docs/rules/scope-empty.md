---
title: Scope Empty
description: Check if the scope exists
---

* Default: `error`

## ❌ Bad

```console
(cli): fix typo
```

## ✅ Good

```console
docs(cli): fix typo
```

## Example

### Scope must exist

```yaml
rules:
    type-empty:
        level: error
```
