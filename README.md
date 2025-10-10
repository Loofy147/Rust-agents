# Rust Agents

This project is a lightweight, asynchronous framework for building and running autonomous agents in Rust. It demonstrates the **ReAct (Reasoning and Acting)** paradigm, where an agent uses a Large Language Model (LLM) to reason about a task and then uses a set of tools to act upon its environment to accomplish a goal.

The agent is now capable of using a real LLM (like OpenAI's GPT models) to solve arbitrary tasks given to it via the command line.

## Architecture

The framework is designed to be simple, extensible, and fully asynchronous, revolving around a few core traits and concepts:

-   **`agent::Agent`**: The primary trait for any agent. It defines the `run` method, which is the main entry point for executing a task. The `ReActAgent` is the provided implementation.
-   **`llm::Llm`**: Represents the Large Language Model that the agent uses for reasoning. The framework includes:
    -   An `OpenAiLlm` client for connecting to the OpenAI API.
    -   A `MockLlm` for deterministic, offline testing.
-   **`tools::Tool`**: A trait for tools that the agent can use to interact with its environment. Each tool has a `name` and an `execute` method. This project includes:
    -   `CodeWriterTool`: To write content to files.
    -   `SystemTool`: To execute arbitrary shell commands.

The application is built on the `tokio` runtime for its asynchronous capabilities and uses `tracing` for structured logging.

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

3.  **Configure the Application:**
    The agent requires an API key to communicate with the OpenAI API.
    -   Copy the example configuration file:
        ```bash
        cp config.toml.example config.toml
        ```
    -   Open `config.toml` and replace `"YOUR_API_KEY"` with your actual OpenAI API key. You can also change the model if you wish.

4.  **Build the Project:**
    Compile the project and its dependencies.
    ```bash
    cargo build --release
    ```

## Usage

To run the agent, use `cargo run` and provide a task as a command-line argument. The task should be enclosed in quotes if it contains spaces.

**Example:**

Here's how to run the original task of generating a program to find the SHA-256 hash of "hello world":

```bash
cargo run --release -- "Generate a program to find the SHA-256 hash of 'hello world'"
```

The agent will use the LLM to reason through the steps: creating a new Cargo project, adding dependencies, writing the Rust code, and finally compiling and running it to get the hash. You will see the agent's entire process logged to the console, including its thoughts, actions, and observations.

## Testing

The project includes a suite of unit and integration tests. To run them, use:
```bash
cargo test
```
The tests use the `MockLlm` to ensure they can run offline and produce deterministic results.