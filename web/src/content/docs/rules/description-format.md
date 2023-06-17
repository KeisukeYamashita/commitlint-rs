---
title: Description format
description: Check if the description format is valid
---

* Default:`ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
    description-format:
        level: error
        format: ^[A-Z].*$
```

## ❌ Bad

```console
feat(cli): added a new flag
```

## ✅ Good

```console
feat(cli): Added a new flag
```

## Example

### Description must start with a capital letter

```yaml
rules:
    description-format:
        level: error
        format: ^[A-Z].*$
```
