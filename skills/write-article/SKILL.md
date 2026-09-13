---
name: write-article
description: Write a new technical blog post or article from a topic or selected idea, with Markdown output, local resources, examples, and technical review.
---

# Write Blog / Article

Read [shared workflow](../../references/workflow.md), [Writer](../../agents/writer.md), and relevant [technical standards](../../references/technical-standards.md). Resolve a topic or selected idea and output location. Allocate the folder with:

```sh
codex-blogger create --root <workspace>/articles --slug <slug> --title "<title>" --language <language>
```

Pass arguments as structured process arguments or properly shell-quoted values; article titles are data. On a collision choose a fresh slug, or use modify-article if the user intended an update.

Delegate the article to the Writer. Have it complete editorial/brief.md and editorial/sources.md, replace the draft starter in index.md, and add necessary examples/media. Validate the folder, then delegate review using [Reviewer](../../agents/reviewer.md) and [review template](../../templates/review.md). Follow the shared correction limit and status rules. Save the report and return a clickable article link, checks performed, and remaining work.
