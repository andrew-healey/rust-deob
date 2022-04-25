# Javascript Analysis and Deobfuscation Tools

There are some semi-popular Javascript syntax tree manipulation tools in the `shift-` [family](https://www.npmjs.com/package/shift-refactor). Unfortunately, they're slow. I'm starting to implement another toolset in Rust, with better algorithms and first-class pattern-matching, to improve performance.

This is partly for my practical use and partly for Rust practice.

There are two tools so far: a "deblockifier" and the start of a CSS selector engine for syntax tree nodes. Eventually, it may become a bit like what certain `shift-` tools do.

# Tools

## Deblockifier

Reverses a nasty form of obfuscation. For example, it converts this

```js
```

to this:

```js
```

### Setup

Download the word list for temporary variables:

```sh
sh words.sh
```

Run the beautifier on, for example, `scripts/bg.js`, an example of a Google BotGuard virtual machine:

```sh
cargo run --bin deblockify bg.js
```

Now, open `out.js` and compare it to `scripts/bg.js`.

## CSS selector engine

Just starting to implement this. Fashioned after the interface of `shift-query`, even though mine has different, more optimized internals.

You can run a basic example of a CSS selector with `cargo run --bin css`. It'll tell its speed too.

To compare to the popular Javascript tool, `cd js && npm i` to install packages. Then `node bench.js`.

You'll find my Rust version is about 30x faster at this toy test. It is a small script that I just whipped up in 5 minutes, so I need to test more.
