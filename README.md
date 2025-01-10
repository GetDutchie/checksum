# checksum

A simple binary that computes checksums for
JSON and YAML files regardless of key ordering.

## Usage

To install run: `cargo install --path .`

```sh
checksum -p <path>
```

```sh
checksum '{"a": 1, "b": 2}' -t json
```

TODOS:

- [ ] Add workflow to handle versioned releases (dutchiebot-releases)
