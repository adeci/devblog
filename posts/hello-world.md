---
title: "Hello World"
date: "2025-10-30"
tags: ["rust", "markdown"]
---

This is the first post to my blog to demonstrate to myself how simple it is to make a post, just commit a markdown file!

## Why I Built This

I want a super **minimal** and fun technical blog where I can:

- Write posts in pure markdown
- Keep everything in version control
- Focus on content, not fancy features or styling

## The RAM Stack

This blog runs on what I dub the RAM stack:

- **Rust** - Memory safe, blazingly fast, compiles to a single binary. No runtime dependencies.
- **Axum** - Rust web framework that's fast and doesn't get in your way. Routes and responses, nothing fancy.
- **Maud** - Compile-time HTML templates in Rust. Type-safe markup that catches errors at build time.

The whole thing compiles to one executable, serves static files, and just works. No build steps, no bundlers, no JavaScript frameworks. Clean! And of course built with Nix.

Pretty cool!
