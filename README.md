# Markdown Preview

![Tests](https://github.com/Mouradost/axum-markdown-preview/actions/workflows/rust.yml/badge.svg)

This is a Markdown previewer build upon [Axum](https://github.com/tokio-rs/axum), [Clap](https://github.com/clap-rs/clap), [Open](https://github.com/Byron/open-rs) and [Markdown-rs](https://github.com/wooorm/markdown-rs)

## Use

To preview a markdown the user can follow the snippet below:

```shell
axum-markdown-preview [PATH_TO_MARKDOWN_FILE]
```

The program will open automatically the default browser with the markdown preview, there are three view mods:

- **Unsafe:** Handle embedded HTML tags -> http://localhost:8080/ (_DEFAULT_).
- **Safe:** Only parse Markdown language -> http://localhost:8080/safe.
- **GitHub:** GitHub like parsing -> http://localhost:8080/gdm.

## To-do

- [ ] Hot-reload
- [ ] More options
