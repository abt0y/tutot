# 🚀 Revised Implementation Plan: Decentralized DeepTutor

This plan defines the architecture for a **fully client-side AI tutoring system** (DeepTutor-equivalent) that runs in the browser via Rust + WebAssembly, using GitHub for identity and persistent storage (LLM Wiki + memvid).

## User Review Required

> [!IMPORTANT]
> **GitHub OAuth (PKCE)**: Pure client-side authentication. We will use a PKCE flow to avoid needing a backend client secret.
>
> **Security**: LLM API keys will be stored in `sessionStorage` or in-memory ONLY.
>
> **Multi-Agent Convergence**: The reasoning loop (Planner -> Tutor -> Critic -> Synthesizer) requires robust error handling in WASM to ensure tasks complete or resume gracefully.

## 1. DeepTutor Feature Mapping

| DeepTutor Feature          | WASM + GitHub Implementation |
| -------------------------- | ------------------------- |
| Multi-agent reasoning loop | Rust async agent orchestrator |
| Tutor-student interaction  | Chat + Guided Pedagogy UI |
| RAG over documents         | LLM Wiki (Persistent Markdown Graph) |
| Knowledge synthesis        | Synthesizer + Maintainer Agents |
| Iterative reasoning        | Agent loop with in-memory context |
| Personalized learning      | `wiki/user_profile.md` + Learning Paths |
| Session tracking           | `memvid` (.mv2) artifact generation |
| Tool usage                 | WASM-native tools + external API calls |

## 2. System Architecture (DeepTutor-Complete)

```mermaid
graph TD
    subgraph "Browser (Rust + WASM)"
        UI[Adaptive Tutor UI]
        Core[Learning Engine]
        Agents[Multi-Agent Orchestrator]
        Pedagogy[Teaching Strategy Engine]
        Memory[Wiki + Session Memory]
        GH[GitHub API Client]
        Memvid[memvid Recorder]
        Search[Wiki Search Engine]
    end

    subgraph "GitHub (User Private Repo)"
        Raw[/raw/ Sources]
        Wiki[/wiki/ Knowledge Base]
        Schema[/schema/ Agent Rules]
        Sessions[/memvid/ Sessions]
    end

    subgraph "External"
        LLM[LLM APIs]
    end

    UI <--> Core
    Core <--> Agents
    Agents <--> Pedagogy
    Agents <--> Memory
    Memory <--> Search
    Agents <--> GH
    Agents <--> LLM
    Core <--> Memvid
    GH <--> Raw
    GH <--> Wiki
    GH <--> Sessions
```

## 3. Pedagogical Intelligence Layer (`pedagogy/`)

This module implements the "brain" of the tutor:
- **Learning Strategies**: Socratic, Direct Instruction, Inquiry-based.
- **Adaptive Difficulty**: Scaling terminology and complexity based on `user_profile.md`.
- **Question Generation**: Dynamic assessment based on wiki content.

### Multi-Agent Registry:
- **Tutor Agent**: Primary interface, manages the dialogue and pedagogical choice.
- **Planner Agent**: Analyzes the gap between user knowledge and target concept.
- **Critic Agent**: Evaluates student responses for misconceptions.
- **Synthesizer Agent**: Merges new knowledge/corrections into the Wiki.
- **Ingest Agent**: Processes raw sources into the knowledge graph.
- **Lint Agent**: Self-maintains wiki consistency (orphan detection, contradictions).

## 4. Storage Model (GitHub = Database)

The repository structure is expanded to support DeepTutor features:

```text
/
├── raw/                # Source files (Immutable)
├── wiki/               # Persistent Long-Term Memory
│   ├── index.md        # Content map
│   ├── log.md          # Session log
│   ├── user_profile.md # Learning history & preferences [NEW]
│   ├── learning_paths/ # Structured curricula [NEW]
│   ├── misconceptions/ # Logged student errors [NEW]
│   ├── explanations/   # Generated student-specific content [NEW]
│   ├── entities/       # People, Places, Tools
│   └── concepts/       # Theories, Ideas
├── schema/             # Agent Personas & Protocols
└── memvid/             # Binary/JSON Session Replays
```

## 5. Reasoning & Learning Loops

```text
User Input
   ↓
Planner Agent (Assess Knowledge Gap)
   ↓
Tutor Agent (Select Strategy & Generate Response)
   ↓
Critic Agent (Optional: Peer review response or evaluate User)
   ↓
Synthesizer → Wiki Update (Long-term memory anchor)
   ↓
Response to User + UI Update (Wiki links, graph nodes)
```

## 6. Rust Crate Structure (WASM Workspace)

- **`core`**: Orchestration and WASM Bindings.
- **`agents`**: Agent traits and specific implementations.
- **`pedagogy`**: Learning models and strategy selectors.
- **`github`**: GitHub API (REST/GraphQL) client.
- **`wiki`**: Markdown processing & graph resolution.
- **`memvid`**: Session lifecycle and `.mv2` recording.
- **`ui`**: Adaptive frontend (likely React/Vue/Vanilla JS bridged to WASM).

## 7. Verification Plan

### Automated
- **Loop Convergence**: Verify Planner -> Tutor -> Critic terminates with valid output.
- **Wiki Integrity**: Test recursive link resolution and frontmatter extraction.
- **WASM Performance**: benchmark commit batching to avoid GitHub rate limits.

### Manual
- **Pedagogy Test**: Verify "Socratic Mode" doesn't give direct answers.
- **Session Replay**: Verify `memvid` artifacts correctly capture the "Wiki Diffs".
- **Deployment**: Verify full flow on GitHub Pages.
