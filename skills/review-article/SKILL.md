---
name: review-article
description: Review an existing technical blog post or article for correctness, sources, code examples, clarity, and media integrity without rewriting it.
---

# Review Blog / Article

Read [shared workflow](../../references/workflow.md), [Reviewer](../../agents/reviewer.md), and relevant [technical standards](../../references/technical-standards.md). Resolve and read the article, resources, and available source notes. For a packaged article run the package validator; for a standalone Markdown file state that package checks are not applicable and inspect its local links manually.

Delegate to the Reviewer with the article and evidence. Save returned findings using [review template](../../templates/review.md): use editorial/review.md for a package, or a unique sibling `<stem>.review.md` for a standalone file. Record the article hash and review mode. Do not alter the article, frontmatter, examples, or assets. Return the report path and the most consequential findings; leave fixes to modify-article.
