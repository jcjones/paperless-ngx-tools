Tools for Paperless-ngx.

`paperless-ngx-upload`: Uploads one or more documents to Paperless-ngx.
`paperless-ngx-tools`: Various helpers for interacting with Paperless-ngx from the CLI.

These are minimum-effort tools, to upload a scanner's output PDF into [Paperless-ngx](https://docs.paperless-ngx.com/), and manipulate it in a basic way.

## Installation
```
cargo install paperless-ngx-tools
```

## Usage
```
Upload a document to Paperless-ngx

Usage: paperless-ngx-upload [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Receipt to upload

Options:
      --url <URL>  URL to use
  -h, --help       Print help
  -V, --version    Print version
```
