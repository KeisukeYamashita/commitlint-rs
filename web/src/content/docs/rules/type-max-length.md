---
title: Type Max Length
description: Check if the type length is less than or equal to the specified length
---

* Default:
  * Level: `ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
  type-max-length:
    level: error
    length: 6
```

## ❌ Bad

```console
feature-for-future(cli): add new flag
```

## ✅ Good

```console
feat(cli): add new flag
```

## Example

### Type length should be less than or equal to 72

```yaml
rules:
  type-max-length:
    level: error
    length: 72
```
