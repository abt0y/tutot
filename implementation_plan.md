# DeepTutor Polish & Production Readiness Plan

This plan addresses the critical feedback regarding the project's structure, completeness, and production readiness. We will transform the current prototype into a more professional and robust monorepo.

## User Review Required

> [!IMPORTANT]
> - **LLM Retries**: I will add `backoff` or a similar retry mechanism to `llm.rs`. This adds a dependency to the `agents` crate.
> - **Root Structure**: I will move some root-level files (e.g., `app.js`, `graph.js`, `style.css`, `index.html`) into a `frontend/` or `web/` directory if you agree, to keep the root clean. For now, I'll keep them but organize them in the `README.md`.

## Proposed Changes

---

### Root Project Organization

#### [MODIFY] [.gitignore](file:///c:/Users/b/Desktop/ai-tutor/.gitignore)
- Add `target/`, `.DS_Store`, `node_modules/`, and other common build artifacts.

#### [NEW] [README.md](file:///c:/Users/b/Desktop/ai-tutor/README.md)
- Provide a high-level overview of the decentralized AI tutor.
- Architecture diagram (text-based or Mermaid).
- Quickstart guide for both Rust and Python/Docker components.

#### [NEW] [Makefile](file:///c:/Users/b/Desktop/ai-tutor/Makefile)
- Add common tasks: `build`, `test`, `run-frontend`, `docker-up`.

#### [MODIFY] [Cargo.toml](file:///c:/Users/b/Desktop/ai-tutor/Cargo.toml)
- Ensure all members are correctly listed and linked.

---

### Rust Agents Refinement (`agents` crate)

#### [NEW] [prompts.rs](file:///c:/Users/b/Desktop/ai-tutor/agents/src/prompts.rs)
- Centralize all system and user prompt templates as constants to avoid magic strings and duplication.

#### [MODIFY] [llm.rs](file:///c:/Users/b/Desktop/ai-tutor/agents/src/llm.rs)
- Implement exponential backoff for LLM API calls.
- Improve error reporting.

#### [MODIFY] [Agent Implementations](file:///c:/Users/b/Desktop/ai-tutor/agents/src/)
- Refactor `planner.rs`, `tutor.rs`, `critic.rs`, and `synthesizer.rs` to use the new `prompts.rs`.
- Improve `synthesizer.rs` path extraction logic.

---

### Quality Assurance

#### [NEW] [tests/agent_tests.rs](file:///c:/Users/b/Desktop/ai-tutor/agents/tests/agent_tests.rs)
- Add integration tests for the agent loop using a mock LLM.

---

## Open Questions

- **Frontend Location**: Should I move the root JS/HTML/CSS files into a `frontend/` directory to keep the root clean?
- **LLM Provider**: The current `GenericLLMClient` assumes OpenAI-compatible `/chat/completions`. Should we add support for others (e.g., Anthropic, local Llama via Ollama)?

## Verification Plan

### Automated Tests
- `cargo test --workspace`: Ensure all crates build and tests pass.
- `cargo fmt --all --check`: Ensure code style is consistent.

### Manual Verification
- Verify that `target/` is no longer tracked by Git.
- Inspect the new `README.md` for clarity and completeness.
- Run a dummy agent loop in a scratch script to verify retry logic.
