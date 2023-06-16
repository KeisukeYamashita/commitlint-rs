---
title: Body Max Length
description: Check if the body length is less than or equal to the specified length
---

* Default:
  * Level: `ignore`
  * Length: `72`

```yaml
rules:
    body-max-length:
        level: error
        length: 4
```

## ❌ Bad

```console
feat(cli): add new flag

Hello, I'm a body of the commit message.
```

## ✅ Good

```console
feat(cli): add new flag

Hey.
```

## Example

```yaml
rules:
    body-max-length:
        level: error
        length: 72
```
