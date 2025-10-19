# Rust Agents

This project is a lightweight framework for building and running autonomous agents in Rust. It demonstrates the **ReAct (Reasoning and Acting)** paradigm, where an agent uses a Large Language Model (LLM) to reason about a task and then uses a set of tools to act upon its environment to accomplish a goal.

The example task implemented in this repository is for an agent to **generate, compile, and run a new Rust program that calculates the SHA-256 hash of the string "hello world"**.

## Architecture

The framework is designed to be simple and extensible, revolving around three core traits:

-   **`agent::Agent`**: The primary trait for any agent. It defines the `run` method, which is the main entry point for executing a task. The `ReActAgent` is the provided implementation of this trait.
-   **`llm::Llm`**: Represents the Large Language Model that the agent uses for reasoning. The framework includes a `MockLlm` for deterministic, offline execution, which simulates the thinking process required for the example task.
-   **`tools::Tool`**: A trait for tools that the agent can use to interact with its environment. Each tool has a `name` and an `execute` method. This project includes:
    -   `CodeWriterTool`: To write content to files.
    -   `SystemTool`: To execute arbitrary shell commands.

The `ReActAgent` orchestrates the entire process in a loop:
1.  It receives a task and formulates a prompt for the LLM.
2.  The LLM returns a `Thought` (its reasoning) and an `Action` (which tool to use and with what arguments).
3.  The agent executes the specified `Action` using the corresponding tool.
4.  The result of the action, called an `Observation`, is fed back into the prompt for the next iteration.
5.  This loop continues until the LLM determines the task is complete and returns a `Finish` action.

## Setup

To get started with this project, you'll need to have the Rust toolchain installed.

1.  **Install Rust:**
    If you don't have Rust, you can install it using `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Clone the Repository:**
    ```bash
    git clone https://github.com/example/rust-agents.git
    cd rust-agents
    ```

3.  **Build the Project:**
    Compile the project and its dependencies.
    ```bash
    cargo build
    ```

## Usage

To run the agent and see it in action, simply use `cargo run`:

```bash
cargo run
```

You will see the agent's entire process printed to the console, including its thoughts, actions, and observations at each step as it works to complete the task. The final output will be the SHA-256 hash calculated by the program that the agent generated.