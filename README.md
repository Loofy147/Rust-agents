# Rust Multi-Agent Framework

This project is a sophisticated, asynchronous framework for building and running multi-agent systems in Rust. It is built on a **Hierarchical Delegation** architecture, where a team of specialized agents collaborates to accomplish complex tasks.

## Architecture

The framework is designed around a central `Orchestrator` that manages a team of supervisor agents. The primary workflow involves three main agent types:

-   **`Orchestrator`**: The top-level agent that receives a high-level goal and delegates it to the appropriate supervisor.
-   **`SupervisorAgent`**: This agent manages a team of specialized worker agents. It receives a task from the orchestrator, determines which worker is best suited for the job, and routes the task accordingly.
-   **`ExecutorAgent`**: A specialized worker agent that performs a specific task using a set of tools. It operates on the **ReAct (Reasoning and Acting)** paradigm, where it reasons about the step, selects a tool, and acts upon the environment.

### Core Components

-   **`orchestrator::Orchestrator`**: The central coordinator that manages the overall workflow. It takes a task and delegates it to a `SupervisorAgent`.
-   **`supervisor::SupervisorAgent`**: Manages a team of worker agents and routes tasks to the appropriate one.
-   **`executor::ExecutorAgent`**: A worker agent that executes a single, well-defined task.
-   **`agent::Agent`**: A generic trait for any agent, defining the common `run` method.
-   **`llm::Llm`**: A trait for Large Language Models. The framework includes two implementations:
    -   `OpenAiLlm`: Connects to the OpenAI API to provide reasoning capabilities to the agents.
    -   `MockLlm`: A mock implementation for deterministic, offline testing.
-   **`tools::Tool`**: A trait for tools that the `ExecutorAgent` can use to interact with its environment.

### Available Tools

The `ExecutorAgent` has access to the following tools:
-   **`CodeWriterTool`**: Writes content to files.
-   **`DirectoryListerTool`**: Lists the contents of a directory.
-   **`FileReaderTool`**: Reads the contents of a file.
-   **`SystemTool`**: Executes arbitrary shell commands.
-   **`WebScraperTool`**: Fetches and parses the text content of a URL.

## Setup

To get started with this project, you'll need to have the Rust toolchain installed.

1.  **Install Rust:**
    If you don't have Rust, you can install it using `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  **Clone the Repository:**
    ```bash
    git clone <repository-url>
    cd rust-multi-agent-framework
    ```

3.  **Set up Environment Variables:**
    You will need an OpenAI API key to run the system with the `OpenAiLlm`. Create a `.env` file in the root of the project:
    ```bash
    echo "OPENAI_API_KEY=your_api_key_here" > .env
    ```

4.  **Configure the Application:**
    You can customize the model used by the LLM in the `config.toml` file. An example is provided in `config.toml.example`.

5.  **Build the Project:**
    Compile the project and its dependencies.
    ```bash
    cargo build
    ```

## Usage

To run the agent system, use `cargo run` with the `--task` flag to specify the goal.

```bash
cargo run -- --task "Scrape the homepage of 'example.com' and save the text content to a file named 'homepage.txt'."
```

### Using the Mock LLM

For testing and development, you can run the system with the mock LLM by adding the `--mock` flag. This does not require an API key.

```bash
cargo run -- --task "your task here" --mock
```

You will see the entire process printed to the console, including the supervisor's routing decisions and the thoughts, actions, and observations of the `ExecutorAgent` at each step.

## Observability

The system is instrumented with [OpenTelemetry](https://opentelemetry.io/) for distributed tracing. To view the traces, you will need to run an OpenTelemetry collector.

### Running a Local Observability Stack

A `docker-compose.yml` file is provided to run a local observability stack, which includes an OpenTelemetry collector and Jaeger for viewing traces.

To start the stack, run:

```bash
docker-compose up -d
```

Once the stack is running, the application will automatically send traces to the collector. You can view the traces by navigating to the Jaeger UI at [http://localhost:16686](http://localhost:16686).