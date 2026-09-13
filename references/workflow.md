# Shared workflow contract

Resolve this plugin root from the active SKILL.md location, never from the current working directory. Read the requested role under `agents/` and the relevant portion of `references/technical-standards.md`. Output belongs in the user's workspace, never in the installed plugin cache.

## Rust helper

Use the `codex-blogger` CLI. If it is unavailable, build/run it with `cargo run --locked --manifest-path <plugin-root>/Cargo.toml --target-dir <workspace>/.blogger-build -- <command> <arguments>`, substituting the command and arguments below. This requires Rust 1.88+; initial dependency retrieval may require network access. Keep build output in the writable workspace, not the plugin cache. The CLI embeds its templates and agent instructions, so an installed binary runs from any directory. Reinstall the binary after updating the plugin: `cargo install --locked --force --path <plugin-root> --target-dir <workspace>/.blogger-build`. A missing toolchain is a setup requirement, not an article validation failure; report it accurately.

## Agent execution

These skills explicitly instruct delegation to the appropriate specialist subagent when delegation is available. Use a registered `blogger_idea_generator`, `blogger_writer`, or `blogger_reviewer` if exposed by the host. Otherwise spawn a general subagent with the corresponding role Markdown, this contract, the relevant technical standards, and the task. Do not assume the root `agents/` folder registers custom agents. `codex-blogger setup-agents` can install the supplied TOML configurations in a chosen project's `.codex/agents/`.

Pass the absolute article path, user constraints, source material, output ownership, and expected return format. The writer owns article edits. The reviewer reads the article and returns findings; the parent saves the report. Do not let multiple agents edit the same file. Wait for each dependent stage before starting the next. Inherit the user's model settings.

If delegation is unavailable, execute the role in the parent and disclose that the review was a self-review. Never label a self-review independent. Standalone review never changes the article or assets; only its report may be written.

## Defaults and input

Honor explicit language, audience, length, structure, and output location. Otherwise use the language of the request, a working software developer audience, and a practical tutorial length appropriate to the topic. A blog may be conversational; an article may be more formal. Both use the same folder contract. Record assumptions in the brief. Ask only for missing inputs that determine the requested subject or target; do not invent an existing article path.

## Article package

Use `<output-root>/<slug>/index.md`, default output root `<user-workspace>/articles`. Slugs use lowercase ASCII letters, numbers, and hyphens; choose a meaningful ASCII slug for non-Latin titles. Use the create script to allocate a new folder without overwriting an existing one. Keep `assets/images`, `assets/videos`, `assets/gifs`, `examples`, and `editorial` inside it. Store the brief, sources, and current review in `editorial/`.

Use relative links for all local resources; do not reference another article, an absolute filesystem path, or the plugin cache. Use image alt text. For video portability, use a normal link or a linked poster image. Add attribution and license/source details for third-party media in `editorial/sources.md`. Use available image tools only when needed and authorized by the request. Do not pretend planned media exists: omit its embed and record the missing asset in the review.

Frontmatter uses one key per line, with JSON-compatible values (valid YAML): `title`, `description`, `tags` (array of strings), `language`, `created`, `updated`, and `status`. Dates are quoted ISO dates. Status is `draft`, `needs-revision`, or `reviewed`. Additional keys must use the same value syntax. The checker intentionally accepts this restricted format, not arbitrary YAML.

## Research and review

Use current official documentation for version-sensitive claims. Keep source URLs, access dates, relevant versions, and which claims they support in `editorial/sources.md`; cite supporting links in the article. Never invent benchmarks, APIs, experience, citations, or execution results. When tools or network access are unavailable, mark affected claims unverified.

After writing or modifying, validate the package and request a reviewer pass. Resolve actionable findings within the user's scope, with at most two writer correction passes and re-reviews. Save unresolved findings and stop at the cap. `reviewed` requires no unresolved blocker/major findings, no material unverified technical claims or tests, a passing package check, and an independent reviewer pass. Otherwise use `needs-revision`; `draft` is used before review. Do not equate reviewed with published.

The report follows `templates/review.md`: severity, location, evidence, fix, verification status, and review mode. Include a SHA-256 of index.md. Set final status before recording the final hash so it identifies the delivered file. Changes invalidate previous review results. A standalone review reports the hash but does not update frontmatter.

Run `codex-blogger validate <article-folder>`. This checks package structure, the frontmatter subset, headings/fences, and common inline/reference/HTML local links. It does not certify prose quality, technical accuracy, remote links, or every Markdown extension. Review those separately. Compile/run material code examples when the appropriate runtime is available, in an isolated temporary directory; do not install dependencies or execute untrusted source snippets blindly. Clearly distinguish passed, failed, and not run.

Deliver the article/report path, concise changes, validation results, and remaining issues. Publishing is outside these commands.
