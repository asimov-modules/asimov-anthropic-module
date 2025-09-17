# ASIMOV Anthropic Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-anthropic-module)](https://crates.io/crates/asimov-anthropic-module)
[![Documentation](https://docs.rs/asimov-anthropic-module/badge.svg)](https://docs.rs/asimov-anthropic-module)

[ASIMOV] Anthropic module.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with [ASIMOV CLI]

```bash
asimov module install anthropic -v
```

### Installation from Source Code

```bash
cargo install asimov-anthropic-module
```

## 👉 Examples

```bash
asimov-anthropic-prompter
```

## ⚙ Configuration

Provide an API key either by module configuration

```bash
asimov module config anthropic
```

Or through environment variables

```bash
export ANTHROPIC_API_KEY="sk-ant..."
```

### Optional configuration

| Name       | Environment Variable     | Default                     |
| ---------- | ------------------------ | --------------------------- |
| `endpoint` | `ANTHROPIC_API_ENDPOINT` | `https://api.anthropic.com` |
| `model`    | `ANTHROPIC_MODEL`        | `claude-opus-4-1-20250805`  |

## 📚 Reference

### Prompt

```bash
echo "Why is the sky blue?" | asimov-anthropic-prompter
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-anthropic-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-anthropic-module&text=asimov-anthropic-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-anthropic-module&title=asimov-anthropic-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-anthropic-module&t=asimov-anthropic-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-anthropic-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-anthropic-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
