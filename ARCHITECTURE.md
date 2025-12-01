# Architecture: Hierarchical Delegation

This document outlines the new architecture for the Rust Multi-Agent Framework, which is based on the Hierarchical Delegation pattern. This design addresses the limitations of the previous orchestration model, improving scalability, resilience, and maintainability.

## 1. Overview

The Hierarchical Delegation architecture organizes agents into teams, with a supervisor agent coordinating each team. A top-level orchestrator manages the supervisors, creating a multi-layered coordination structure that balances centralized control with distributed execution.

## 2. Key Components

-   **Top-Level Orchestrator:** The primary entry point for user tasks. Its main responsibility is to understand the high-level goal and delegate it to the appropriate supervisor agent.
-   **Supervisor Agents:** Each supervisor manages a team of specialized worker agents. It receives a task from the orchestrator, determines which worker in its team is best suited for the job, and routes the task accordingly. It is also responsible for aggregating results from its workers and reporting back to the orchestrator.
-   **Worker Agents:** These are specialized agents (formerly the `ExecutorAgent`) that perform the actual work. Each worker has a specific set of tools and is responsible for executing a single, well-defined step.
-   **Teams:** A logical grouping of worker agents managed by a single supervisor. For example, a "DataCollectionTeam" might include a `WebScraperAgent` and a `FileReaderAgent`.

## 3. Data Flow

The new data flow is as follows:

```
[User Task]
     |
     v
[Orchestrator] -> (Delegates task to the appropriate team)
     |
     v
[Supervisor Agent] -> (Routes task to the appropriate worker)
     |
     v
[Worker Agent] -> (Executes the task using its tools)
     |
     v
[Tool] -> (Interacts with the environment)
     |
     v
[Observation] -> (Result of the tool execution)
     |
     v
[Worker Agent] -> (Receives the observation)
     |
     v
[Supervisor Agent] -> (Aggregates the result)
     |
     v
[Orchestrator] -> (Receives the final result)
     |
     v
[Final Answer]
```

## 4. Component Responsibilities

| Component | Responsibilities |
|---|---|
| **Orchestrator** | - Receives the initial high-level task. <br> - Determines which team is best suited for the task. <br> - Delegates the task to the appropriate `SupervisorAgent`. <br> - Receives the final result from the `SupervisorAgent` and returns it to the user. |
| **SupervisorAgent** | - Manages a team of `WorkerAgent`s. <br> - Receives a task from the `Orchestrator`. <br> - Uses an LLM to determine which worker in its team should execute the task. <br> - Routes the task to the selected `WorkerAgent`. <br> - Aggregates the results from the `WorkerAgent`s. |
| **WorkerAgent** | - Executes a single, well-defined task. <br> - Uses the ReAct (Reasoning and Acting) loop to select a tool and execute it. <br> - Returns the result of the tool execution (the observation) to the `SupervisorAgent`. |

## 5. Benefits of this Architecture

-   **Scalability:** Teams can operate in parallel, and worker agents can be scaled horizontally.
-   **Resilience:** Failures are isolated at the team or worker level, preventing system-wide outages.
-   **Maintainability:** The clear separation of concerns makes the system easier to understand, maintain, and extend.
-   **Flexibility:** New teams and agents can be added without modifying the core orchestrator logic.
