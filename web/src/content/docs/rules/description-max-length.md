---
title: Description Max Length
description: Check if the description length is less than or equal to the specified length
---

* Default:
  * Level: `ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
    description-max-length:
        level: error
        length: 12
```

## ❌ Bad

```console
feat(cli): add new flag for brand new feature
```

## ✅ Good

```console
feat(cli): add help flag
```

## Example

### Description length should be less than or equal to 72

```yaml
rules:
    description-max-length:
        level: error
        length: 72
```
