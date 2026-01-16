# devblog

**Live at: [adeci.dev](https://adeci.dev)**

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

A minimal technical blog built with the RAM stack (Rust, Axum, Maud).

Powered by Nix and deployed to a computer in my closet as the lord intended.

## How to run

**Development:**

- Local clone
  ```bash
  nix run .#dev
  ```

**Production:**

- Local clone
  ```bash
  nix run .
  ```
- Remote
  ```bash
  nix run github:adeci/devblog
  ```

## Writing posts

Drop markdown files in `posts/` with YAML frontmatter:

```markdown
---
title: "Post Title"
date: "YYYY-MM-DD"
tags: ["rust", "nix"]
---

# Your content here
```

That's it. Commit and deploy.
