---
title: Scope format
description: Check if the scope format is valid
---

* Default:`ignore`

In this page, we will use the following commit message as an example.

```yaml
rules:
  scope-format:
    level: error
    format: ^[a-z]*$
```

## ❌ Bad

```console
feat(Cli): added a new flag
```

## ✅ Good

```console
feat(cli): added a new flag
```

## Example

### Scope must start with a lower letter

```yaml
rules:
  scope-format:
    level: error
    format: ^[a-z]*$
```
