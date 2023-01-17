# Composer

## Description

Composer is a cli tool which used to Generate WorkFlow Wasm and Test Provider Hooks.

This CLI Compose of Two commands

- `Generate` - Used to generate WASM Binary from YAML
- `Test` - Used to test the provider Hooks

![alt text](./images/composer.png)

## Commands

### Generate

```
composer generate -c config_file -o outpath_path
```

![alt text](./images/generate.png)

### Test

```
composer test
```

### composer test will test all the hooks & run unit test cases & return the result.

![alt text](./images/test.png)

## References

[cobra](https://github.com/spf13/cobra)

### License

[label](https://www.apache.org/licenses/LICENSE-2.0)
Licensed under [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0)
