---
trigger: always_on
glob: code-quality-guide
description: enforce code formatting, linting, and static typing after code changes
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/code-quality-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Code Quality Rule

**Rules:**

- After writing or modifying code, always run formatters and linters.
- Execute formatting tools and fix all formatting issues.
- Execute linting tools and fix all lint/type errors until passed.
- For Python projects, use standard tools (e.g. `ruff`, `black`, `mypy`) via the project's dependency manager.
- All tools must be installed and run inside the project's virtual environment.

**Violations:**

- Skipping formatting or linting.
- Ignoring formatter or linter errors.
- Running tools outside the virtual environment.
- Missing standard quality tools in dependencies.
