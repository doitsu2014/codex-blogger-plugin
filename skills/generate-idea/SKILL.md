---
name: generate-idea
description: Generate ideas for technical blog posts or articles focused on engineering problems and solutions. Use when the user wants topics or article briefs.
---

# Generate Idea

Read [shared workflow](../../references/workflow.md) and [Idea Generator](../../agents/idea-generator.md). Delegate topic development to the Idea Generator using the user's area, audience, constraints, and any existing idea list. Read the relevant [technical standards](../../references/technical-standards.md).

Return the requested number of ranked ideas (default five). Save a Markdown idea list under `<workspace>/ideas/` with a unique date/topic filename; never replace an existing list without an explicit update request. Each idea must be actionable as input to write-article. Do not create article folders or start writing unless requested.
