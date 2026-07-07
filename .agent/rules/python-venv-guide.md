---
trigger: glob
glob: python-venv-guide
globs: *.py
description: enforce Python 3, virtual environments, and Poetry for dependency management
agentic:
  generated_by: agentic
  source: "areas/software/full-stack/rules/python-venv-guide.md"
  repository: "https://github.com/sawrus/agent-guides"
  created_by: "v0.6.0"
  updated_by: "v0.6.0"
---

# Python Venv & Poetry Rule

**Rules:**

- Use Python 3 in a project-specific venv (`.venv_projectname`).
- Activate venv before running code/tests.
- Initialize project structure with `src/` directory and place all code files inside it.
- Install and manage all dependencies via Poetry (`poetry add/install/update`).
- Commit `pyproject.toml` and `poetry.lock`.
- Ignore `.venv_projectname/` in Git.

**Violations:**

- Running code outside venv.
- Installing packages globally.
- Not using Poetry for dependency management.
