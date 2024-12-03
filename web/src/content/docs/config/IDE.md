---
title: IDE
description: Guide how to setup your IDE to work with commitlint
---

Commitlint offers schema by supporting [JSON schema](https://json-schema.org/) so that you can configure your IDE to work with Commitlint and have better developer experience.

:::tip

If you want to pin the schema to a specific version, you can configure the version in the URL.

```console
https://github.com/KeisukeYamashita/commitlint-rs/releases/download/v0.2.0/schema.json
```

In this case, the schema is pinned to version `0.2.0`.

:::

## Visual Studio Code

Configure your [Visual Studio Code](https://code.visualstudio.com/) to work with Commitlint.

### Edit in `settings.json`

Update your user `settings.json` or workspace settings (`/.vscode/settings.json`) to configure the schema.

#### JSON

```json
"json.schemas": [
  {
    "fileMatch": [
      ".commitlintrc",
      ".commitlintrc.json"
    ],
    "url": "https://github.com/KeisukeYamashita/commitlint-rs/releases/latest/download/schema.json"
  }
]
```

#### YAML

Associating schemas with YAMLs are supported by the [YAML language server](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml).

```json
"yaml.schemas": {
    "https://github.com/KeisukeYamashita/commitlint-rs/releases/latest/download/schema.json": [
        ".commitlintrc",
        ".commitlintrc.yaml",
        ".commitlint.yml"
    ]
}
```

### Specify schema in the configuration file

```json
{
  "$schema": "https://github.com/KeisukeYamashita/commitlint-rs/releases/download/v0.2.0/schema.json",
  "rules": {}
}
```

#### JSON

Add the following comment in your `.commitlintrc` or `.commitlintrc.json` file.

```json
{
  "$schema": "https://github.com/KeisukeYamashita/commitlint-rs/releases/latest/download/schema.json",
  "rules": {}
}
```

#### YAML

Associating schemas with YAMLs are supported by the [YAML language server](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml). Add the following comment in your `.commitlintrc`, `.commitlintrc.yaml` or `.commitlintrc.yml` file.

```yaml
# yaml-language-server: $schema=https://github.com/KeisukeYamashita/commitlint-rs/releases/latest/download/schema.json
rules:
```
