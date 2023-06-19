---
title: Scope Max Length
description: Check if the scope length is less than or equal to the specified length
---

* Default:
  * Level: `ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
    scope-max-length:
        level: error
        length: 6
```

## ❌ Bad

```console
feat(super important product): add new flag
```

## ✅ Good

```console
feat(cli): add new flag
```

## Example

### Description length should be less than or equal to 72

```yaml
rules:
    scope-max-length:
        level: error
        length: 72
```
