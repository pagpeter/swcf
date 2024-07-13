# Rust SWC deobfuscation PoC for Cloudflare

Much credit goes to https://github.com/levi-nz/vercel-anti-bot

Some parts are rewritten from https://github.com/wwhtrbbtt/deob-transformations

## Features

This repo aims to provide some tooling for analysing the cloudflare "I am under attack mode" (IUAM) browser fingerprinting challenge.

The challenge consists of an obfuscated JavaScript file (init script), obfuscated with a custom version of the open-source obfuscator.io framework, which loads and executes a custom Virtual Machine (VM). The virtualized program then executes a bunch of "sub-challenges" to fingerprint your browser environment. The results gets send back to cloudflare, and either you get a cf_clearance cookie or not. (This is simplified - the actual request flow is a bit more complex.)

This makes analysis very hard. However, to start, you need to deobfuscate the init script and load the bytecode for the VM. This is what this repo aims to automate.

## Usage of the init script deobfuscator

```sh
# Developement
$ ./data/getinput.sh # Getting challenge script
$ cargo run --bin deobfuscator data/input.js

# Production
$ cargo build --release
$ ./target/release/deobfuscator data/input.js
```

## Usage of the "solver" (unfinished and outdated)

```sh
# Developement
$ cargo run --bin solver

# Production
$ cargo build --release
$ ./target/release/solver
```

## Why

I wanted to learn rust and swc, and wanted to see the benefits of using it over babel in NodeJS.

## Performance

The performance is worse than I thought, but that probably has to do with my very bad, completely unoptimized code. However, it's still 20-30x faster than similar code written in NodeJS/babel.
When compiled with the release flag, it takes ~55ms for reading, parsing, deobfuscating, marshalling the AST back to valid JavaScript and writing everything to file.

## TLS-Client-API

- https://github.com/bogdanfinn/tls-client-api/releases
- https://bogdanfinn.gitbook.io/open-source-oasis/standalone-api-application/download
