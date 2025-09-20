# RustChain Agent API Specification

## Overview

The RustChain Agent API provides autonomous reasoning agents that implement the ReAct (Reasoning + Acting) pattern for intelligent problem-solving. Agents can access tools, maintain memory, and collaborate in multi-agent systems to solve complex tasks.

## Version

- **API Version**: 1.0.0
- **RustChain Version**: 0.1.0+
- **Specification**: GitHub Spec Kit compliant

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    RustChain Agent System                   │
├─────────────────────────────────────────────────────────────┤
│  Agent<'a>                                                  │
│  ├── MemoryStore (Context & Learning)                      │
│  ├── ToolRegistry (Actions & Capabilities)                 │
│  ├── LLMBackend (Reasoning & Decision Making)              │
│  └── AgentState (Execution Status)                         │
├─────────────────────────────────────────────────────────────┤
│  MultiAgentSystem                                           │
│  ├── AgentTrait (Polymorphic Agent Interface)              │
│  ├── CommunicationChannel (Inter-Agent Messaging)          │
│  └── Specialized Agents (Research, Planning, Execution)    │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Agent<'a>

The main autonomous reasoning agent implementing the ReAct pattern.

```rust
pub struct Agent<'a> {
    pub name: String,
    pub memory: &'a mut dyn MemoryStore,
    pub tools: &'a ToolRegistry,
    pub llm: &'a dyn LLMBackend,
    pub max_iterations: usize,
    pub verbose: bool,
    state: AgentState,
}
```

#### Constructor Methods

##### `new(name, memory, tools, llm) -> Self`

Creates a new agent with the specified components.

**Parameters**:
- `name: String` - Agent identifier
- `memory: &'a mut dyn MemoryStore` - Memory store for context and learning
- `tools: &'a ToolRegistry` - Available tools for actions
- `llm: &'a dyn LLMBackend` - LLM for reasoning and decision making

**Returns**: `Agent<'a>`

**Example**:
```rust
let agent = Agent::new(
    "TaskAgent".to_string(),
    &mut memory_store,
    &tool_registry,
    &llm_backend,
);
```

##### `with_max_iterations(max: usize) -> Self`

Sets maximum reasoning iterations (default: 10).

**Parameters**:
- `max: usize` - Maximum iterations before termination

**Returns**: `Self`

##### `with_verbose(verbose: bool) -> Self`

Enables verbose execution logging.

**Parameters**:
- `verbose: bool` - Enable detailed logging

**Returns**: `Self`

#### Core Methods

##### `run(objective: &str) -> Result<String, RustChainError>`

Main agent execution loop implementing the ReAct pattern.

**Parameters**:
- `objective: &str` - The task or goal for the agent to accomplish

**Returns**: `Result<String, RustChainError>` - Final answer or execution result

**Execution Flow**:
1. **Observe** - Analyze current state and context
2. **Think** - Reason about the objective and next steps
3. **Decide** - Choose an action based on reasoning
4. **Act** - Execute the chosen action (tool use, answer, or request info)
5. **Repeat** - Continue until objective is met or max iterations reached

**Example**:
```rust
let agent = Agent::new("TaskAgent".to_string(), &mut memory, &tools, &llm)
    .with_max_iterations(15)
    .with_verbose(true);

let result = agent.run("Analyze the sales data and create a summary report").await?;
println!("Agent result: {}", result);
```

#### Internal Methods

##### `observe() -> Result<String, RustChainError>`

Observes the current state by retrieving context from memory.

##### `think(observation: &str, objective: &str) -> Result<String, RustChainError>`

Uses LLM to reason about the current situation and next steps.

##### `decide_action(thought: &str) -> Result<AgentAction, RustChainError>`

Analyzes the thought and decides on the next action to take.

##### `use_tool(tool: &str, input: &str) -> Result<String, RustChainError>`

Executes a tool with the given input and returns the result.

### 2. AgentAction

Enumeration of possible agent actions.

```rust
pub enum AgentAction {
    UseTool { tool: String, input: String },
    Answer(String),
    RequestMoreInfo(String),
    Think,
}
```

#### Variants

- **`UseTool`** - Execute a tool with specified input
- **`Answer`** - Provide final answer to the objective
- **`RequestMoreInfo`** - Request additional information (currently logs to memory)
- **`Think`** - Continue reasoning without external action

### 3. AgentState

Agent execution state tracking.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Idle,
    Running,
    Waiting,
    Completed,
    Failed,
}
```

#### State Transitions

```
Idle → Running → (Waiting ↔ Running)* → (Completed | Failed)
```

### 4. AgentTrait

Polymorphic interface for different agent types.

```rust
pub trait AgentTrait: Send + Sync {
    fn name(&self) -> &str;
    async fn process_task(&mut self, task: &str, context: &str) -> Result<String, RustChainError>;
    async fn receive_message(&mut self, from: &str, message: &str) -> Result<(), RustChainError>;
}
```

#### Methods

##### `name() -> &str`

Returns the agent's name identifier.

##### `process_task(task: &str, context: &str) -> Result<String, RustChainError>`

Processes a task with given context.

##### `receive_message(from: &str, message: &str) -> Result<(), RustChainError>`

Receives and processes inter-agent messages.

### 5. MultiAgentSystem

Collaborative system for multiple agents working together.

```rust
pub struct MultiAgentSystem {
    agents: HashMap<String, Box<dyn AgentTrait>>,
    communication_channel: CommunicationChannel,
}
```

#### Methods

##### `new() -> Self`

Creates a new multi-agent system.

##### `add_agent(agent: Box<dyn AgentTrait>)`

Adds an agent to the system.

**Parameters**:
- `agent: Box<dyn AgentTrait>` - Agent implementation to add

##### `collaborate_on(task: &str) -> Result<String, RustChainError>`

Coordinates multiple agents to work on a task collaboratively.

**Parameters**:
- `task: &str` - The collaborative task

**Returns**: `Result<String, RustChainError>` - Synthesized result from all agents

### 6. Specialized Agent Types

#### ResearchAgent

Specialized for research and information gathering tasks.

```rust
pub struct ResearchAgent {
    name: String,
    sources: Vec<String>,
}
```

#### PlanningAgent

Specialized for strategic planning and task decomposition.

```rust
pub struct PlanningAgent {
    name: String,
    strategies: Vec<String>,
}
```

#### ExecutionAgent

Specialized for task execution and implementation.

```rust
pub struct ExecutionAgent {
    name: String,
    capabilities: Vec<String>,
}
```

### 7. CommunicationChannel

Inter-agent communication system.

```rust
pub struct CommunicationChannel {
    messages: Vec<Message>,
    context: String,
}
```

#### Methods

##### `new() -> Self`

Creates a new communication channel.

##### `send_message(from: &str, to: &str, content: &str)`

Sends a message between agents.

##### `get_context() -> &str`

Retrieves current communication context.

##### `add_context(context: &str)`

Adds context to the communication channel.

## Usage Examples

### Single Agent Execution

```rust
use rustchain::core::agent::Agent;
use rustchain::core::memory::InMemoryStore;
use rustchain::core::tools::ToolRegistry;
use rustchain::core::llm::create_default_llm_manager;

// Setup components
let mut memory = InMemoryStore::new(3600); // 1 hour TTL
let tools = ToolRegistry::new();
let llm = create_default_llm_manager()?;

// Create and configure agent
let mut agent = Agent::new(
    "DataAnalyst".to_string(),
    &mut memory,
    &tools,
    &*llm,
)
.with_max_iterations(20)
.with_verbose(true);

// Execute task
let result = agent.run("Analyze the quarterly sales data and identify key trends").await?;
```

### Multi-Agent Collaboration

```rust
use rustchain::core::agent::{MultiAgentSystem, ResearchAgent, PlanningAgent, ExecutionAgent};

let mut multi_agent_system = MultiAgentSystem::new();

// Add specialized agents
multi_agent_system.add_agent(Box::new(ResearchAgent {
    name: "Researcher".to_string(),
    sources: vec!["database".to_string(), "api".to_string()],
}));

multi_agent_system.add_agent(Box::new(PlanningAgent {
    name: "Planner".to_string(),
    strategies: vec!["divide_and_conquer".to_string()],
}));

multi_agent_system.add_agent(Box::new(ExecutionAgent {
    name: "Executor".to_string(),
    capabilities: vec!["data_processing".to_string(), "report_generation".to_string()],
}));

// Collaborative task execution
let result = multi_agent_system.collaborate_on(
    "Create a comprehensive market analysis report"
).await?;
```

### Custom Agent Implementation

```rust
use rustchain::core::agent::AgentTrait;
use async_trait::async_trait;

pub struct CustomAgent {
    name: String,
    specialty: String,
}

#[async_trait]
impl AgentTrait for CustomAgent {
    fn name(&self) -> &str {
        &self.name
    }

    async fn process_task(&mut self, task: &str, context: &str) -> Result<String, RustChainError> {
        // Custom task processing logic
        Ok(format!(
            "Processed '{}' using {} specialty with context: {}",
            task, self.specialty, context
        ))
    }

    async fn receive_message(&mut self, from: &str, message: &str) -> Result<(), RustChainError> {
        // Custom message handling
        println!("{} received from {}: {}", self.name, from, message);
        Ok(())
    }
}
```

### Agent with Tool Integration

```rust
// Register tools for agent use
let mut tools = ToolRegistry::new();
tools.register_tool("web_search", Box::new(WebSearchTool::new()))?;
tools.register_tool("data_analysis", Box::new(DataAnalysisTool::new()))?;
tools.register_tool("report_generator", Box::new(ReportGeneratorTool::new()))?;

// Agent will automatically use these tools during reasoning
let mut agent = Agent::new("ResearchAssistant".to_string(), &mut memory, &tools, &llm);

let result = agent.run("Research competitor pricing and create a comparison report").await?;
```

## ReAct Pattern Implementation

The RustChain Agent implements the ReAct (Reasoning + Acting) pattern:

### 1. Observation Phase
- Retrieves current context from memory
- Analyzes previous actions and results
- Builds situational awareness

### 2. Reasoning Phase
- Uses LLM to analyze the current situation
- Considers the objective and available information
- Plans the next logical step

### 3. Action Phase
- Decides on specific action (tool use, answer, or info request)
- Executes the chosen action
- Stores results in memory for future reasoning

### 4. Learning Phase
- Updates memory with new information
- Builds context for subsequent iterations
- Improves decision-making over time

## Performance Characteristics

- **Initialization**: ~50ms (agent setup)
- **Iteration Time**: 200ms-2s (depends on LLM and tool complexity)
- **Memory Usage**: ~10MB base + ~1MB per iteration
- **Scalability**: Supports concurrent agents with proper resource management
- **Tool Latency**: Varies by tool (file operations ~10ms, API calls ~100ms-1s)

## Error Handling

All agent methods return `Result<T, RustChainError>` with specific error types:

```rust
use rustchain::core::error::{ExecutionError, MemoryError, ToolError};

match agent.run("objective").await {
    Ok(result) => println!("Success: {}", result),
    Err(RustChainError::Execution(ExecutionError::MaxIterationsReached)) => {
        println!("Agent reached maximum iterations without completing objective");
    },
    Err(RustChainError::Memory(MemoryError::StorageFailed { .. })) => {
        println!("Memory operation failed");
    },
    Err(RustChainError::Tool(ToolError::NotFound { .. })) => {
        println!("Required tool not available");
    },
    Err(e) => println!("Agent error: {}", e),
}
```

## Feature Flags

The agent system requires the `agent` feature flag:

```toml
[dependencies]
rustchain = { version = "0.1.0", features = ["agent"] }
```

Additional features for enhanced functionality:
- `memory` - Advanced memory stores
- `tools` - Extended tool registry
- `llm` - LLM backend integration

## Integration Patterns

### Mission Integration

```rust
// Agents can be used within mission steps
let mission_step = MissionStep {
    step_type: StepType::Agent,
    parameters: json!({
        "agent_name": "DataProcessor",
        "objective": "Process quarterly sales data",
        "max_iterations": 15,
        "verbose": true
    }),
    // ... other step configuration
};
```

### Chain Integration

```rust
// Agents can be part of processing chains
let chain = Chain::new("agent_processing_chain")
    .add_step(Box::new(DataPreparationStep::new()))
    .add_step(Box::new(AgentProcessingStep::new("Analyst")))
    .add_step(Box::new(ResultValidationStep::new()));
```

### Tool Ecosystem

Agents automatically integrate with the RustChain tool ecosystem:
- **File Operations**: Read, write, edit files
- **HTTP Requests**: API interactions
- **Data Processing**: Analysis and transformation
- **External Services**: Database queries, web searches
- **Custom Tools**: Domain-specific implementations

## Security Considerations

1. **Tool Access Control**: Agents only access registered tools
2. **Memory Isolation**: Agent memory is isolated from other agents
3. **Resource Limits**: Maximum iterations prevent infinite loops
4. **Input Validation**: All inputs are validated before processing
5. **Error Containment**: Agent failures don't affect other system components

## Best Practices

### Agent Design
- Use specific, measurable objectives
- Configure appropriate iteration limits
- Provide relevant tools for the task domain
- Use verbose mode during development and debugging

### Multi-Agent Systems
- Design agents with complementary capabilities
- Use clear communication protocols
- Implement proper task coordination
- Monitor resource usage across agents

### Memory Management
- Use appropriate TTL for memory stores
- Clean up completed agent contexts
- Implement memory size limits for long-running agents
- Store relevant context for future iterations

## Future Enhancements

- **Learning Agents**: Persistent learning across sessions
- **Hierarchical Planning**: Multi-level objective decomposition
- **Agent Marketplaces**: Discoverable, reusable agent implementations
- **Performance Optimization**: Parallel reasoning and action execution
- **Advanced Communication**: Rich message protocols between agents
- **Visual Debugging**: Agent reasoning visualization and debugging tools

---

*This specification follows the GitHub Spec Kit standards for API documentation and is maintained alongside the RustChain codebase.*