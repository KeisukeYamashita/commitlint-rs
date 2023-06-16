---
title: Configuration
description: Guide how to configure commitlint
---

Commitlint can be configured in many ways. You can use the default configuration or you can specify your own configuration file.

## Default configuration

If you don't specify any configuration file and you don't have any commitlint configuration files in your environment, the CLI will use the default configurations of each rules.

See the [default rules](/rules/default) page for details.

## Location

Commitlint will look for configuration in the following places.

### Default

If no flag (see next section) is specified, the Commitlint will look for configuration in the following places **in the current working directory**:

* `.commitlintrc` (JSON or YAML file)
* `.commitlintrc.yml` (YAML file)
* `.commitlintrc.yaml` (YAML file)

:::tip

Note that it is searched in the order written above and the first one found is loaded. Therefore, if you have `.commitlintrc` and `.commitlintrc.yml` in the same directory, the `.commitlintrc` will be loaded and the second one will be ignored.

:::

### Using the flag

Configuration file can be specified by using the `--config` flag or the short `-g` flag.

```console
# Using --config flag
$ commitlint --config path/to/.commitlintrc.yml

# Using -g flag
$ commitlint -g path/to/.commitlintrc.yml
```

If you specify a file and the file is not found, Commitlint will throw an error.

## Debug configuration

You can use the `--print-config` flag to print the configuration that will be used by Commitlint.

```console
$ commitlint --print-config
rules:
    description-empty: # Description must not be empty
        level: warn
    subject-empty: # Subject line must not be empty
        level: error
    type-empty: # Type must not be empty
        level: error
```
