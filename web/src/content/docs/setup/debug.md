---
title: Motivation
description: Debug your configurations
---

You maybe wondering how you can debug your configurations. This is a common question and we have a few options for you to consider.

## Debugging how a rule works

Debug your configuration using stdin and below:

```console
echo "feat(other): debug" | commitlint
```

## Debugging your configuration

You can debug how your configuration is being loaded and what rules are being used by running the following command:

```console
commitlint --print-config
```
