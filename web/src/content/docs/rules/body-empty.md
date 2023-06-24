---
title: Body Empty
description: Check if the body exists
---

* Default: `ignore`

## ❌ Bad

```console
feat(cli): add new flag
```

## ✅ Good

```console
feat(cli): add new flag

Add new flag --help for https://github.com/KeisukeYamashita/commitlint-rs/issues/20
```

## Example

```yaml
rules:
  body-empty:
    level: error
```
