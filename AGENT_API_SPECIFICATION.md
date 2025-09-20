# 🤖 RustChain Agent API Specification for Shimmy Integration

**Version**: 2.0.0  
**Date**: 2025-01-20  
**Purpose**: Complete API specification for Shimmy TUI to integrate with RustChain Agent backend

---

## 🎯 **API OVERVIEW**

The Agent API provides a Claude Code-compatible interface that allows Shimmy TUI to:
- **Chat with AI agents** using RustChain's multi-provider LLM system
- **Execute tools** for file operations, code analysis, git operations, and system commands  
- **Manage context** and conversation history
- **Access project understanding** via PUNCH integration
- **Perform real-time operations** with sub-second response times

---

## 🚀 **BASE CONFIGURATION**

```
Base URL: http://localhost:8080
Content-Type: application/json
Authentication: None (local-only communication)
```

---

## 📋 **CORE ENDPOINTS**

### **1. Agent Message Processing**
**Primary endpoint for AI conversation**

```http
POST /agent/message
```

**Request:**
```json
{
  "message": "Help me analyze this codebase",
  "context": {
    "current_directory": "/path/to/project",
    "session_id": "sess_12345678",
    "conversation_history": ["previous", "messages"],
    "active_files": ["/src/main.rs", "/Cargo.toml"]
  },
  "options": {
    "model": "shimmy:phi3-mini",
    "temperature": 0.7,
    "max_tokens": 2000,
    "stream": false
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "response": "I'll help you analyze this Rust codebase...",
    "session_id": "sess_12345678",
    "message_id": "msg_87654321", 
    "tokens_used": 145,
    "processing_time_ms": 847,
    "tools_called": [],
    "agent_actions": [
      {
        "type": "analysis",
        "description": "Analyzed project structure",
        "confidence": 0.95
      }
    ]
  },
  "timestamp": "2025-01-20T15:30:00Z"
}
```

### **2. Tool Execution**
**Execute specific tools with parameters**

```http
POST /agent/tool
```

**Request:**
```json
{
  "tool": "file_read",
  "parameters": {
    "path": "/src/main.rs",
    "encoding": "utf-8"
  },
  "context": {
    "session_id": "sess_12345678",
    "current_directory": "/path/to/project"
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "tool": "file_read",
    "result": {
      "path": "/src/main.rs",
      "content": "fn main() {\n    println!(\"Hello, world!\");\n}",
      "size_bytes": 41,
      "encoding": "utf-8",
      "last_modified": "2025-01-20T10:15:00Z"
    },
    "execution_time_ms": 23
  },
  "timestamp": "2025-01-20T15:30:01Z"
}
```

### **3. Context Management**
**Get and set conversation context**

```http
GET /agent/context/{session_id}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "session_id": "sess_12345678",
    "created_at": "2025-01-20T15:25:00Z",
    "last_activity": "2025-01-20T15:30:01Z",
    "message_count": 5,
    "context": {
      "current_directory": "/path/to/project",
      "active_files": ["/src/main.rs"],
      "project_type": "rust_cargo",
      "conversation_summary": "User is analyzing a Rust project structure",
      "available_tools": ["file_read", "file_write", "git_status", "punch_analyze"]
    },
    "conversation_history": [
      {
        "id": "msg_1",
        "role": "user", 
        "content": "Analyze this project",
        "timestamp": "2025-01-20T15:25:00Z"
      },
      {
        "id": "msg_2",
        "role": "assistant",
        "content": "I'll analyze your Rust project...",
        "timestamp": "2025-01-20T15:25:05Z"
      }
    ]
  }
}
```

---

## 🔧 **SPECIALIZED ENDPOINTS**

### **File Operations**
```http
POST /agent/file/read      # Read file contents
POST /agent/file/write     # Write/create files  
POST /agent/file/edit      # Edit file with patches
POST /agent/file/delete    # Delete files safely
GET  /agent/file/list      # List directory contents
```

### **Code Analysis (PUNCH Integration)**
```http
POST /agent/analyze/discover   # Discover project components
POST /agent/analyze/query      # Query codebase patterns
POST /agent/analyze/deps       # Analyze dependencies
POST /agent/analyze/quality    # Code quality assessment
```

### **Git Operations** 
```http
GET  /agent/git/status         # Git repository status
POST /agent/git/commit         # Create commits
POST /agent/git/push           # Push changes
GET  /agent/git/log            # Commit history
POST /agent/git/branch         # Branch operations
```

### **System Commands**
```http
POST /agent/system/execute     # Safe command execution
GET  /agent/system/processes   # List processes
GET  /agent/system/info        # System information
```

---

## 🛠️ **AVAILABLE TOOLS**

### **Core Tools**
- `file_read` - Read file contents
- `file_write` - Create/update files
- `file_edit` - Apply patches to files  
- `file_delete` - Delete files with confirmation
- `directory_list` - List directory contents
- `command_execute` - Execute shell commands safely

### **Development Tools**
- `punch_discover` - Analyze project architecture
- `punch_query` - Search codebase patterns
- `punch_stats` - Get project statistics
- `git_status` - Repository status
- `git_commit` - Create commits
- `git_branch` - Branch operations

### **Analysis Tools**
- `code_analyze` - Static code analysis
- `security_scan` - Security vulnerability scan  
- `performance_profile` - Performance analysis
- `dependency_audit` - Dependency security check

---

## 📊 **ERROR HANDLING**

### **Standard Error Response**
```json
{
  "success": false,
  "error": "Tool execution failed: File not found",
  "error_code": "FILE_NOT_FOUND",
  "error_details": {
    "path": "/nonexistent/file.rs",
    "tool": "file_read",
    "suggestion": "Check if the file path exists"
  },
  "timestamp": "2025-01-20T15:30:02Z"
}
```

### **Common Error Codes**
- `INVALID_REQUEST` - Malformed request
- `TOOL_NOT_FOUND` - Requested tool doesn't exist
- `FILE_NOT_FOUND` - File operation failed
- `PERMISSION_DENIED` - Insufficient permissions
- `TIMEOUT` - Operation timed out
- `LLM_ERROR` - AI model error
- `RATE_LIMIT` - Too many requests

---

## 🚀 **PERFORMANCE REQUIREMENTS**

### **Response Time Targets**
- **Agent Message**: <1000ms (sub-second)
- **Tool Execution**: <500ms (fast operations)
- **File Operations**: <200ms (immediate)
- **Context Retrieval**: <100ms (instant)

### **Concurrent Handling**
- **Sessions**: 50+ simultaneous sessions
- **Requests**: 1000+ requests/minute
- **Memory**: <500MB RAM usage
- **CPU**: <25% CPU usage

---

## 🔒 **SECURITY FEATURES**

### **Path Safety**
- Path traversal protection (`../` blocked)
- System directory access denied
- Windows reserved filename protection
- Sandboxed file operations

### **Command Safety**
- Dangerous command blocking
- Argument sanitization  
- Process isolation
- Timeout enforcement

### **Request Validation**
- JSON schema validation
- Parameter type checking
- Size limit enforcement
- Rate limiting

---

## 📝 **INTEGRATION EXAMPLES**

### **Shimmy TUI Usage**
```rust
// Shimmy client code example
let client = RustChainClient::new("http://localhost:8080");

// Send chat message
let response = client.send_message(&AgentMessage {
    message: "Help me refactor this function",
    context: current_context(),
    options: MessageOptions::default(),
}).await?;

// Execute tool
let file_content = client.execute_tool("file_read", json!({
    "path": "/src/main.rs"
})).await?;

// Get project context
let context = client.get_context(&session_id).await?;
```

### **Full Workflow Example**
```
1. User: "Analyze this Rust project" 
   → POST /agent/message

2. Agent: Uses punch_discover tool
   → POST /agent/tool (punch_discover)

3. Agent: Reads key files  
   → POST /agent/tool (file_read)

4. Agent: Provides analysis response
   → Response with comprehensive analysis

5. Context updated automatically
   → GET /agent/context shows full conversation
```

---

## 🎯 **SHIMMY INTEGRATION SPECIFICATION**

### **Required Client Implementation**
```rust
pub struct RustChainClient {
    base_url: String,
    http_client: reqwest::Client,
    session_id: String,
}

impl RustChainClient {
    pub async fn send_message(&self, msg: &AgentMessage) -> Result<AgentResponse>;
    pub async fn execute_tool(&self, tool: &str, params: Value) -> Result<ToolResult>;
    pub async fn get_context(&self, session_id: &str) -> Result<SessionContext>;
}
```

### **TUI Integration Points**
- **Chat Panel**: Uses `/agent/message` for conversation
- **File Panel**: Uses `/agent/file/*` for file operations  
- **Context Panel**: Uses `/agent/context/*` for state management
- **Output Panel**: Displays tool execution results and logs

### **Startup Sequence**
1. Shimmy checks if RustChain server running on port 8080
2. If not running, starts `rustchain server --agent-mode --port 8080`  
3. Creates new session via `/agent/context`
4. Initializes project context with current directory
5. Ready for user interaction

---

## ✅ **API COMPLETION STATUS**

- [x] **Specification Design** - Complete API design
- [ ] **Core Endpoints** - `/agent/message`, `/agent/tool`, `/agent/context`
- [ ] **File Operations** - All file management endpoints
- [ ] **PUNCH Integration** - Code analysis endpoints  
- [ ] **Git Operations** - Version control endpoints
- [ ] **System Commands** - Safe command execution
- [ ] **Error Handling** - Comprehensive error responses
- [ ] **Performance Testing** - Sub-second response validation
- [ ] **Documentation** - Complete API documentation

---

**🎯 Next Step**: Implement the core `/agent/message` endpoint with full agent reasoning and tool execution capabilities.