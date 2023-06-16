---
title: Default Rules
description: List of the default rules
---

If you don't specify any configuration file and you don't have any commitlint configuration files in your environment, the CLI will use the default configurations of each rules describes in this page.

:::tip

You can also check the default values on the page of each rule.

:::

```yaml
rules:
    description-empty: # Description must not be empty
        level: warn
    scope-empty: # Scope must not be empty
        level: error
    subject-empty: # Subject line should exist
        level: error
    type-empty: # Type must not be empty
        level: error
```
