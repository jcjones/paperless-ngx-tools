Tools for Paperless-ngx.

`paperless-ngx-upload`: Uploads one or more documents to Paperless-ngx.
`paperless-ngx-tools`: Various helpers for interacting with Paperless-ngx from the CLI.

These are minimum-effort tools, to upload a scanner's output PDF into [Paperless-ngx](https://docs.paperless-ngx.com/), and manipulate it in a basic way.

## Installation
```
cargo install paperless-ngx-tools
```

## Configuration

Configuration is in TOML format in a file in a [Project Dir](https://crates.io/crates/directories), such as:

    Lin: /home/alice/.config/rs.paperless-ngx-tools
    Win: C:\Users\Alice\AppData\Roaming\Foo Corp\rs.paperless-ngx-tools\config
    Mac: /Users/Alice/Library/Application Support/rs.paperless-ngx-tools

```
→ cat '/Users/pug/Library/Application Support/rs.paperless-ngx-tools/default-config.toml'
url = "https://paperless.example"
auth = "ffffffffffffffffffffffffffffffffffffffff"
```

You can also use `paperless-ngx-tools --url ... --auth ... store` to set the config.

## Usage
```console
→ paperless-ngx-upload --help
Upload a document to Paperless-ngx

Usage: paperless-ngx-upload [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Receipt to upload

Options:
      --url <URL>  URL to use
  -h, --help       Print help
  -V, --version    Print version
```

```console
→ paperless-ngx-tools --help
Interact with Paperless-ngx

Usage: paperless-ngx-tools [OPTIONS] [COMMAND]

Commands:
  list-correspondents     list correspondants
  list-documents          list documents, optionally by filter mechanism
  list-document-ids       list document IDs, optionally by filter mechanism
  migrate-correspondents  move documents from one correspondent to another
  delete-correspondent    delete a correspondent
  store                   Stores the --auth and --url to the config file
  help                    Print this message or the help of the given subcommand(s)

Options:
      --url <URL>    URL to use
      --auth <AUTH>  Auth token to use
  -n, --noop         Do not make changes
  -h, --help         Print help
  -V, --version      Print version
```