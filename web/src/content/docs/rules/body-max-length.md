---
title: Body Max Length
description: Check if the body length is less than or equal to the specified length
---

* Default:
  * Level: `ignore`
  * Length: `72`

In this page, we will use the following commit message as an example.

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

### Body length should be less than or equal to 72

```yaml
rules:
    body-max-length:
        level: error
        length: 72
```
