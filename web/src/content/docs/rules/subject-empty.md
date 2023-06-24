---
title: Subject Empty
description: Check if the subject exists
---

* Default: `error`

## ❌ Bad

```console


Body of the commit
```

## ✅ Good

```console
docs(cli): fix typo

Body of the commit
```

## Example

### Subject must exist

```yaml
rules:
  subject-empty:
    level: error
```
