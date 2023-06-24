---
title: Description Empty
description: Check if the description exists
---

* Default: `error`

## ❌ Bad

```console
feat(cli): 
```

## ✅ Good

```console
feat(cli): add new flag
```

## Example

### Description must exist

```yaml
rules:
  description-empty:
    level: error
```
