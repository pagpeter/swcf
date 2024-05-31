# Rust SWC deobfuscation PoC for Cloudflare

Much credit goes to https://github.com/levi-nz/vercel-anti-bot

Some parts are rewritten from https://github.com/wwhtrbbtt/deob-transformations

## Features

At the moment, it only deobfuscates some parts of cloudflares obfuscation: the string scrambling, the proxy functions / strings, and a bit more.

## Usage

```sh
# Developement
$ cargo run --bin deobfuscator data/input.js

# Production
$ cargo build --release
$ ./target/release/deobfuscator data/input.js
```

## Why

I wanted to learn rust and swc, and wanted to see the benefits of using it over babel in NodeJS.

## Performance

The performance is worse than I thought, but that probably has to do with my very bad, completely unoptimized code. However, it's still 20-30x faster than similar code written in NodeJS/babel.
When compiled with the release flag, it takes ~55ms for reading, parsing, deobfuscating, marshalling the AST back to valid JavaScript and writing everything to file.
