# Codex Blogger Plugin

Create technical ideas, write and modify Markdown articles, and review them with three specialist agent roles. Supports C#, Go, Rust, JavaScript/TypeScript, Angular, React, mobile apps, AI agents, AI workflows, and AI coding.

## Commands

Select the skill from the Codex skill picker, or use its `$` invocation. If another plugin has the same name, select the skill under **Codex Blogger Plugin**.

| Skill | Example request |
| --- | --- |
| `$generate-idea` | Generate five Go concurrency article ideas for backend developers. |
| `$write-article` | Write a practical article from idea 2, with a runnable example. |
| `$modify-article` | Update articles/my-topic/index.md to explain cancellation. |
| `$review-article` | Review articles/my-topic/index.md for technical correctness. |

These are Codex skills, not shell commands or custom slash commands. Natural-language requests can also select the skills. Idea generation creates a Markdown list under `ideas/`; it does not automatically select a topic.

## Article output

```text
articles/my-topic/
├── index.md
├── assets/
│   ├── images/
│   ├── videos/
│   └── gifs/
├── examples/
└── editorial/
    ├── brief.md
    ├── sources.md
    └── review.md
```

Each article is a portable folder. Local links stay inside it. Videos use normal Markdown links or linked poster images. The output root, language, tone, and length can be specified in the request. Defaults are the current workspace's `articles/`, the request's language, and a practical developer audience.

Frontmatter uses JSON-compatible values within YAML delimiters:

```yaml
---
title: "Practical Go cancellation"
description: "Propagate cancellation through a worker."
tags: ["go", "concurrency"]
language: "en"
created: "2026-09-13"
updated: "2026-09-13"
status: "draft"
---
```

Quote strings and dates, and use inline arrays. The validator deliberately does not accept arbitrary YAML (such as block scalars or multiline lists).

## Agents and review

- **Idea Generator** proposes concrete engineering problems and demonstrable solutions.
- **Writer** researches, writes, and modifies articles and examples.
- **Reviewer** examines technical claims, code, sources, and presentation independently.

Skills delegate using the role instructions in `agents/`. On hosts without named custom agents, they pass those instructions to a general subagent. If delegation is unavailable, they disclose self-review. Shared technical standards require current primary sources and honest reporting of checks not run; the plugin does not bundle a trained model or guarantee technical accuracy.

Writing and modification include package checks and review, with at most two correction passes. `reviewed` requires an independent pass, no unresolved major/blocking issues or material unverified checks, and passing package validation. Other reviewed drafts use `needs-revision`. Review alone writes a report and leaves the article unchanged. Reports identify the article using SHA-256. No command publishes content.

## Local setup

The plugin manifest is `.codex-plugin/plugin.json`; all four skills are under `skills/`. Installation requires a Codex host supporting plugins. Once this source is registered in the personal marketplace, install with:

```sh
codex plugin add codex-blogger-plugin@personal
```

For initial registration, use Codex's plugin-creator workflow with this repository as the source and the default personal marketplace. It creates the marketplace entry and source mapping without replacing other plugins. The install command above assumes that registration has completed and the marketplace is named `personal`.

Start a **new task** after installation to pick up the skills. Plugin installation does not automatically register standalone named-agent TOML files. The skill-based delegation works without them. To register named agents in a content project, run from this repository:

```sh
cargo install --path . --locked --target-dir /tmp/codex-blogger-build
codex-blogger setup-agents --project /absolute/path/to/content-project
```

Ensure Cargo's binary directory (normally `~/.cargo/bin`) is on PATH. This generates three self-contained configurations in the project's `.codex/agents/`. It preserves differing existing agent files and inherits model settings. Repeat runs with identical definitions are safe. Start a new task in that project after setup. Agent instructions travel with the generated files; regenerate intentionally when updating the plugin's roles. Templates and agent instructions are embedded in the binary, so it can run outside this repository. Reinstall the CLI with `cargo install --path . --locked --force --target-dir /tmp/codex-blogger-build` when upgrading this plugin.

## Helpers and checks

The helpers and tests are written in Rust. Building requires Rust 1.88+ and Cargo; the first build may download crates. Cargo.lock pins dependency versions. The compiled CLI requires no Python, API key, server, or mandatory media-generation integration.

```sh
codex-blogger create --root /path/to/articles --slug my-topic --title "My topic" --language en
codex-blogger validate /path/to/articles/my-topic
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

Without installing the binary, use `cargo run --locked --manifest-path /path/to/plugin/Cargo.toml --target-dir /path/to/workspace/.blogger-build -- validate /path/to/articles/my-topic` (or another subcommand). This keeps build artifacts outside the installed plugin cache.

The create command produces an intentionally unfinished starter. Validation exits nonzero until it is replaced. Creation refuses collisions. Validation checks metadata, title headings, code fences, escaping symlinks, common inline/reference/HTML resource links, and missing local files. It does not fetch remote links, execute examples, validate every Markdown extension, or certify editorial quality. Run the appropriate compiler/tests separately and record results in `editorial/sources.md`.

Contributor validation also uses the official plugin-creator and skill-creator validators supplied by Codex. Those external development tools are separate from this Rust plugin and its runtime requirements.

## Layout

`skills/` contains the four command workflows; `agents/` contains role instructions; `references/` contains the shared workflow and domain standards; `templates/` contains editorial templates; `src/` contains the Rust CLI and library; `tests/` covers package integrity, agent setup, and CLI behavior.

Licensed under MIT. See [LICENSE](LICENSE).
