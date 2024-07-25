---
title: Footers Empty
description: Check if the footers exists
---

* Default: `error`

## ❌ Bad

```console
feat(cli): user logout handler
```

## ✅ Good

```console
feat(cli): add new flag

Link: https://keisukeyamashita.github.io/commitlint-rs/
```

## Example

### Footers must exist

```yaml
rules:
  footers-empty:
    level: error
```
