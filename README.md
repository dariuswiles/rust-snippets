# Rust Snippets

I'm using this repo to collect short Rust programs that I've written to solve specific problems
when working on larger projects. e.g., to add multi-threading. I'm still learning Rust so the
code can undoubtedly be improved. The repo is only intended for my use, but it's public on the
off-chance it is useful to others.

## Repo Structure and Use

Each snippet is in its own Cargo crate, which also means its own sub-directory. If you are at the
top level and using Bash, a snippet can be run with:
```bash
cargo run -p thread-worker
```

Alternatively, you can ```cd``` into a crate and use:
```bash
cargo run
```

## Snippets

| Directory Name | Snippet Description |
| --- | --- |
| thread-worker | A simple, multi-threaded worker. [Chapter 20.3 of The Rust Programming Language](https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html) has a more generic example along similar lines. |
