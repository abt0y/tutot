# DeepTutor: Decentralized AI Tutoring System

DeepTutor is a state-of-the-art, decentralized AI tutoring system designed to provide a personalized, persistent learning experience. It leverages a multi-agent reasoning loop, a distributed knowledge graph (Wiki), and browser-based persistence.

## 🚀 Key Features

- **Multi-Agent Architecture**: Collaborative agents (Planner, Tutor, Critic, Synthesizer) orchestrate the learning journey.
- **Client-Side First**: Built with Rust & WebAssembly to run primarily in the browser for privacy and decentralization.
- **Persistent Knowledge Wiki**: An LLM-maintained markdown wiki that grows with the student.
- **D3.js Visualization**: Interactive knowledge graph to visualize learning progress.
- **Memory & Context**: Persistence via `memvid` and GitHub integration.

## 🏗️ Architecture

The system is organized as a monorepo containing multiple Rust crates and a Python/Docker application suite.

### Rust Workspace Members
- **`agents/`**: Core reasoning loop and LLM communication logic.
- **`core/`**: Shared types and utilities.
- **`wiki/`**: Knowledge graph persistence and markdown processing.
- **`pedagogy/`**: Teaching strategies and educational models.
- **`memvid/`**: Persistence and local state management.
- **`github/`**: Remote persistence and synchronization.
- **`ui/`**: WebAssembly bindings for the frontend.

### Frontend
- **`index.html` / `app.js`**: The primary user interface for the AI tutor.
- **`graph.js`**: Knowledge graph visualization using D3.js.

### Python Backend (`DeepTutor/`)
- Orchestration layer and API services supporting the agent ecosystem.

## 🛠️ Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for frontend build)
- [Docker](https://www.docker.com/get-started) (for Python services)

### Build and Run

1.  **Build Rust Crates**:
    ```bash
    cargo build --workspace
    ```

2.  **Start Python Services**:
    ```bash
    cd DeepTutor
    docker compose up -d
    ```

3.  **Run Frontend**:
    Serve the root directory using any local development server (e.g., `npx serve .` or Live Server).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
