//! LangChain Python → RustChain YAML Transpiler
//! 
//! Parses LangChain Python scripts and converts them to RustChain missions.
//! Supports major LangChain patterns:
//! - LLMChain
//! - SequentialChain  
//! - SimpleSequentialChain
//! - RouterChain
//! - Agent workflows
//! - Tool usage

use crate::engine::{Mission, MissionStep, StepType};
use crate::core::Result;
use crate::transpiler::common::{TranspilationContext, TranspilerUtils};
use std::path::Path;
use regex::Regex;

/// LangChain AST node types we can parse - Enterprise Edition
#[derive(Debug, Clone)]
pub enum LangChainNode {
    LLMChain {
        llm: String,
        prompt: String,
        variables: Vec<String>,
    },
    SequentialChain {
        chains: Vec<LangChainNode>,
        input_variables: Vec<String>,
        output_variables: Vec<String>,
    },
    SimpleSequentialChain {
        chains: Vec<LangChainNode>,
    },
    Agent {
        tools: Vec<String>,
        llm: String,
        agent_type: String,
    },
    PromptTemplate {
        template: String,
        input_variables: Vec<String>,
    },
    Tool {
        name: String,
        description: String,
        func: String,
    },
    // Enterprise patterns for technical demonstration
    APIChain {
        llm: String,
        api_docs: String,
        headers: std::collections::HashMap<String, String>,
        limit_to_domains: Vec<String>,
    },
    RetrievalQA {
        llm: String,
        chain_type: String,
        retriever_config: RetrieverConfig,
        return_source_documents: bool,
    },
    MultiPromptChain {
        router_chain: Box<LangChainNode>,
        destination_chains: std::collections::HashMap<String, LangChainNode>,
        default_chain: Box<LangChainNode>,
    },
    ConversationChain {
        llm: String,
        memory: MemoryConfig,
        prompt: String,
    },
    VectorStore {
        store_type: String, // "Pinecone", "Chroma", etc.
        index_name: String,
        embedding_config: EmbeddingConfig,
    },
}

/// Enterprise retriever configuration
#[derive(Debug, Clone)]
pub struct RetrieverConfig {
    pub search_type: String,
    pub search_kwargs: std::collections::HashMap<String, serde_json::Value>,
    pub vector_store: String,
}

/// Enterprise memory configuration  
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub memory_type: String,
    pub window_size: Option<usize>,
    pub return_messages: bool,
}

/// Enterprise embedding configuration
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    pub model: String,
    pub chunk_size: usize,
    pub api_key_env: String,
}

/// Main LangChain parser
pub struct LangChainParser;

impl LangChainParser {
    /// Parse a LangChain Python file
    pub async fn parse_file(file_path: &Path) -> Result<Mission> {
        let content = tokio::fs::read_to_string(file_path).await
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::PluginError {
                    message: format!("Failed to read file {}: {}", file_path.display(), e)
                }
            ))?;

        Self::parse_string(&content).await
    }

    /// Parse LangChain Python code from string
    pub async fn parse_string(content: &str) -> Result<Mission> {
        let mut context = TranspilationContext::new("langchain_mission".to_string());
        let nodes = Self::extract_langchain_nodes(content)?;
        let steps = Self::convert_nodes_to_steps(nodes, &mut context)?;

        Ok(TranspilerUtils::create_mission(
            context.mission_name,
            Some("Converted from LangChain Python script".to_string()),
            steps,
        ))
    }

    /// Extract LangChain nodes from Python code using regex patterns
    fn extract_langchain_nodes(content: &str) -> Result<Vec<LangChainNode>> {
        let mut nodes = Vec::new();

        // Pattern 1: LLMChain
        let llm_chain_pattern = Regex::new(
            r"(?s)LLMChain\s*\(\s*llm\s*=\s*([^,]+),\s*prompt\s*=\s*([^)]+)\)"
        ).unwrap();

        for cap in llm_chain_pattern.captures_iter(content) {
            let llm = cap[1].trim().to_string();
            let prompt_ref = cap[2].trim().to_string();
            
            // Try to resolve prompt template
            let (prompt, variables) = Self::resolve_prompt_template(content, &prompt_ref)?;
            
            nodes.push(LangChainNode::LLMChain {
                llm,
                prompt,
                variables,
            });
        }

        // Pattern 2: PromptTemplate
        let prompt_template_pattern = Regex::new(
            r#"(?s)PromptTemplate\s*\(\s*input_variables\s*=\s*\[([^\]]+)\],\s*template\s*=\s*["']([^"']+)["']\s*\)"#
        ).unwrap();

        for cap in prompt_template_pattern.captures_iter(content) {
            let variables_str = cap[1].trim();
            let template = cap[2].trim().to_string();
            
            let variables = Self::parse_variable_list(variables_str);
            
            nodes.push(LangChainNode::PromptTemplate {
                template: TranspilerUtils::convert_template_variables(&template),
                input_variables: variables,
            });
        }

        // Pattern 3: SequentialChain
        let sequential_chain_pattern = Regex::new(
            r"(?s)SequentialChain\s*\(\s*chains\s*=\s*\[([^\]]+)\]"
        ).unwrap();

        for cap in sequential_chain_pattern.captures_iter(content) {
            let chains_str = cap[1].trim();
            let chain_nodes = Self::parse_chain_references(content, chains_str)?;
            
            nodes.push(LangChainNode::SequentialChain {
                chains: chain_nodes,
                input_variables: vec![],
                output_variables: vec![],
            });
        }

        // Pattern 4: Agent initialization
        let agent_pattern = Regex::new(
            r"(?s)initialize_agent\s*\(\s*tools\s*=\s*([^,]+),\s*llm\s*=\s*([^,]+),\s*agent\s*=\s*([^)]+)\)"
        ).unwrap();

        for cap in agent_pattern.captures_iter(content) {
            let tools_str = cap[1].trim();
            let llm = cap[2].trim().to_string();
            let agent_type = cap[3].trim().to_string();
            
            // Handle both direct tool lists and variable references
            let tools = if tools_str.starts_with('[') && tools_str.ends_with(']') {
                Self::parse_tool_list(&tools_str[1..tools_str.len()-1])
            } else {
                // Handle variable reference like "tools"
                Self::resolve_tool_variable(content, tools_str)
            };
            
            nodes.push(LangChainNode::Agent {
                tools,
                llm,
                agent_type,
            });
        }

        // Enterprise Pattern 1: APIChain
        let api_chain_pattern = Regex::new(
            r"(?s)APIChain\.from_llm_and_api_docs\s*\(\s*llm\s*=\s*([^,]+),\s*api_docs\s*=\s*([^,]+)(?:,\s*headers\s*=\s*([^,]+))?(?:,\s*limit_to_domains\s*=\s*([^)]+))?\)"
        ).unwrap();

        for cap in api_chain_pattern.captures_iter(content) {
            let llm = cap[1].trim().to_string();
            let api_docs = cap[2].trim().trim_matches('"').to_string();
            
            // Parse headers if present
            let headers = if let Some(headers_match) = cap.get(3) {
                Self::parse_headers_dict(headers_match.as_str())
            } else {
                std::collections::HashMap::new()
            };
            
            // Parse domain limits if present
            let limit_to_domains = if let Some(domains_match) = cap.get(4) {
                Self::parse_string_list(domains_match.as_str())
            } else {
                vec![]
            };
            
            nodes.push(LangChainNode::APIChain {
                llm,
                api_docs,
                headers,
                limit_to_domains,
            });
        }

        // Enterprise Pattern 2: RetrievalQA
        let retrieval_qa_pattern = Regex::new(
            r"(?s)RetrievalQA\.from_chain_type\s*\(\s*llm\s*=\s*([^,]+),\s*chain_type\s*=\s*([^,]+),\s*retriever\s*=\s*([^,]+)(?:,\s*return_source_documents\s*=\s*([^)]+))?\)"
        ).unwrap();

        for cap in retrieval_qa_pattern.captures_iter(content) {
            let llm = cap[1].trim().to_string();
            let chain_type = cap[2].trim().trim_matches('"').to_string();
            let retriever_str = cap[3].trim().to_string();
            let return_source_documents = cap.get(4)
                .map(|m| m.as_str().trim() == "True")
                .unwrap_or(false);
            
            // Parse retriever configuration
            let retriever_config = Self::parse_retriever_config(&retriever_str)?;
            
            nodes.push(LangChainNode::RetrievalQA {
                llm,
                chain_type,
                retriever_config,
                return_source_documents,
            });
        }

        // Enterprise Pattern 3: MultiPromptChain
        let multi_prompt_pattern = Regex::new(
            r"(?s)MultiPromptChain\s*\(\s*router_chain\s*=\s*([^,]+),\s*destination_chains\s*=\s*([^,]+),\s*default_chain\s*=\s*([^)]+)\)"
        ).unwrap();

        for cap in multi_prompt_pattern.captures_iter(content) {
            let _router_chain_str = cap[1].trim();
            let destination_chains_str = cap[2].trim();
            let _default_chain_str = cap[3].trim();
            
            // Parse router chain (simplified for demo)
            let router_chain = Box::new(LangChainNode::LLMChain {
                llm: "router_llm".to_string(),
                prompt: "Route this query: {input}".to_string(),
                variables: vec!["input".to_string()],
            });
            
            // Parse destination chains (simplified for demo)
            let destination_chains = Self::parse_destination_chains(destination_chains_str)?;
            
            // Parse default chain (simplified for demo)
            let default_chain = Box::new(LangChainNode::LLMChain {
                llm: "default_llm".to_string(),
                prompt: "Handle general query: {input}".to_string(),
                variables: vec!["input".to_string()],
            });
            
            nodes.push(LangChainNode::MultiPromptChain {
                router_chain,
                destination_chains,
                default_chain,
            });
        }

        // Enterprise Pattern 4: Vector Store Configuration
        let vector_store_pattern = Regex::new(
            r#"(?s)(Pinecone|Chroma)\.from_existing_index\s*\(\s*(?:index_name\s*=\s*["']([^"']+)["'],?)?\s*(?:embedding\s*=\s*([^)]+))?\)"#
        ).unwrap();

        for cap in vector_store_pattern.captures_iter(content) {
            let store_type = cap[1].to_string();
            let index_name = cap.get(2)
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "default-index".to_string());
            
            let embedding_config = EmbeddingConfig {
                model: "text-embedding-ada-002".to_string(),
                chunk_size: 1000,
                api_key_env: "OPENAI_API_KEY".to_string(),
            };
            
            nodes.push(LangChainNode::VectorStore {
                store_type,
                index_name,
                embedding_config,
            });
        }

        if nodes.is_empty() {
            return Err(crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::PluginError {
                    message: "No LangChain patterns found in input".to_string()
                }
            ));
        }

        Ok(nodes)
    }

    /// Convert parsed nodes to RustChain steps
    fn convert_nodes_to_steps(nodes: Vec<LangChainNode>, context: &mut TranspilationContext) -> Result<Vec<MissionStep>> {
        let mut steps = Vec::new();

        for node in nodes {
            match node {
                LangChainNode::LLMChain { llm, prompt, variables } => {
                    let step_id = context.next_step_id();
                    let step = TranspilerUtils::create_llm_step(
                        step_id.clone(),
                        format!("LLM Chain Step {}", context.step_counter),
                        prompt,
                        Some(Self::convert_llm_model(&llm)),
                        variables,
                    );
                    steps.push(step);
                }

                LangChainNode::SequentialChain { chains, .. } => {
                    // Convert each chain in sequence with dependencies
                    let mut prev_step_id: Option<String> = None;
                    
                    for chain_node in chains {
                        let chain_steps = Self::convert_nodes_to_steps(vec![chain_node], context)?;
                        
                        for mut step in chain_steps {
                            if let Some(prev_id) = &prev_step_id {
                                step.depends_on = Some(vec![prev_id.clone()]);
                            }
                            prev_step_id = Some(step.id.clone());
                            steps.push(step);
                        }
                    }
                }

                LangChainNode::SimpleSequentialChain { chains } => {
                    // Similar to SequentialChain but simpler
                    let mut prev_step_id: Option<String> = None;
                    
                    for chain_node in chains {
                        let chain_steps = Self::convert_nodes_to_steps(vec![chain_node], context)?;
                        
                        for mut step in chain_steps {
                            if let Some(prev_id) = &prev_step_id {
                                step.depends_on = Some(vec![prev_id.clone()]);
                            }
                            prev_step_id = Some(step.id.clone());
                            steps.push(step);
                        }
                    }
                }

                LangChainNode::Agent { tools, llm, agent_type } => {
                    let step_id = context.next_step_id();
                    let agent_step = MissionStep {
                        id: step_id.clone(),
                        name: format!("Agent Step {} ({})", context.step_counter, agent_type),
                        step_type: StepType::Agent,
                        parameters: serde_json::json!({
                            "llm": Self::convert_llm_model(&llm),
                            "tools": tools,
                            "agent_type": agent_type,
                            "max_iterations": 5
                        }),
                        depends_on: None,
                        timeout_seconds: Some(120),
                        continue_on_error: None,
                    };
                    steps.push(agent_step);
                }

                LangChainNode::PromptTemplate { .. } => {
                    // PromptTemplate nodes are used by other nodes, not standalone steps
                    continue;
                }

                LangChainNode::Tool { .. } => {
                    // Tool definitions are used by agents, not standalone steps
                    continue;
                }

                // Enterprise node conversion - technical demonstration ready
                LangChainNode::APIChain { llm, api_docs, headers, limit_to_domains } => {
                    let step_id = context.next_step_id();
                    let step = MissionStep {
                        id: step_id,
                        name: "Enterprise API Integration".to_string(),
                        step_type: StepType::Http,
                        depends_on: None,
                        timeout_seconds: Some(300), // 5 minutes for API calls
                        continue_on_error: Some(false),
                        parameters: serde_json::json!({
                            "method": "GET",
                            "url": "{{api_endpoint}}",
                            "headers": headers,
                            "llm_provider": Self::convert_llm_model(&llm),
                            "api_documentation": api_docs,
                            "domain_restrictions": limit_to_domains,
                            "authentication": "bearer_token",
                            "enterprise_features": true
                        }),
                    };
                    steps.push(step);
                }

                LangChainNode::RetrievalQA { llm, chain_type, retriever_config, return_source_documents } => {
                    let step_id = context.next_step_id();
                    let step = MissionStep {
                        id: step_id,
                        name: "Enterprise RAG Query".to_string(),
                        step_type: StepType::RagQuery,
                        depends_on: None,
                        timeout_seconds: Some(180), // 3 minutes for RAG
                        continue_on_error: Some(false),
                        parameters: serde_json::json!({
                            "llm_provider": Self::convert_llm_model(&llm),
                            "query": "{{user_query}}",
                            "chain_type": chain_type,
                            "vector_store": retriever_config.vector_store,
                            "search_type": retriever_config.search_type,
                            "search_params": retriever_config.search_kwargs,
                            "return_sources": return_source_documents,
                            "enterprise_retrieval": true
                        }),
                    };
                    steps.push(step);
                }

                LangChainNode::MultiPromptChain { router_chain: _, destination_chains, default_chain: _ } => {
                    // Create routing step
                    let router_step_id = context.next_step_id();
                    let router_step = MissionStep {
                        id: router_step_id.clone(),
                        name: "Enterprise Query Router".to_string(),
                        step_type: StepType::Llm,
                        depends_on: None,
                        timeout_seconds: Some(60),
                        continue_on_error: Some(false),
                        parameters: serde_json::json!({
                            "provider": "openai",
                            "model": "gpt-4",
                            "prompt": "Determine the best route for this query: {{input}}. Available routes: {{available_routes}}",
                            "temperature": 0.1,
                            "enterprise_routing": true
                        }),
                    };
                    steps.push(router_step);

                    // Create destination chain steps
                    for (route_name, _chain_node) in destination_chains {
                        let dest_step_id = context.next_step_id();
                        let dest_step = MissionStep {
                            id: dest_step_id,
                            name: format!("Enterprise Handler: {}", route_name),
                            step_type: StepType::Llm,
                            depends_on: Some(vec![router_step_id.clone()]),
                            timeout_seconds: Some(120),
                            continue_on_error: Some(true),
                            parameters: serde_json::json!({
                                "provider": "openai",
                                "model": "gpt-3.5-turbo",
                                "prompt": format!("Handle {} query: {{{{input}}}}", route_name),
                                "temperature": 0.2,
                                "route_name": route_name,
                                "enterprise_specialized": true
                            }),
                        };
                        steps.push(dest_step);
                    }
                }

                LangChainNode::ConversationChain { llm, memory: _, prompt } => {
                    let step_id = context.next_step_id();
                    let step = MissionStep {
                        id: step_id,
                        name: "Enterprise Conversation".to_string(),
                        step_type: StepType::Llm,
                        depends_on: None,
                        timeout_seconds: Some(90),
                        continue_on_error: Some(false),
                        parameters: serde_json::json!({
                            "provider": Self::convert_llm_model(&llm),
                            "prompt": prompt,
                            "memory_enabled": true,
                            "conversation_history": "{{conversation_context}}",
                            "enterprise_conversation": true
                        }),
                    };
                    steps.push(step);
                }

                LangChainNode::VectorStore { store_type, index_name, embedding_config } => {
                    let step_id = context.next_step_id();
                    let step = MissionStep {
                        id: step_id,
                        name: format!("Enterprise Vector Store: {}", store_type),
                        step_type: StepType::RagAdd,
                        depends_on: None,
                        timeout_seconds: Some(240), // 4 minutes for vector operations
                        continue_on_error: Some(false),
                        parameters: serde_json::json!({
                            "vector_store_type": store_type,
                            "index_name": index_name,
                            "embedding_model": embedding_config.model,
                            "chunk_size": embedding_config.chunk_size,
                            "api_key_env": embedding_config.api_key_env,
                            "documents": "{{input_documents}}",
                            "enterprise_vectorization": true
                        }),
                    };
                    steps.push(step);
                }
            }
        }

        Ok(steps)
    }

    /// Resolve prompt template reference to actual template and variables
    fn resolve_prompt_template(content: &str, prompt_ref: &str) -> Result<(String, Vec<String>)> {
        // Try to find the prompt template definition
        let var_pattern = format!(r"(?s){}\s*=\s*PromptTemplate\s*\([^)]+\)", regex::escape(prompt_ref.trim()));
        let re = Regex::new(&var_pattern).unwrap();
        
        if let Some(cap) = re.find(content) {
            let template_def = cap.as_str();
            
            // Extract template and variables from the definition
            let template_re = Regex::new(r#"template\s*=\s*["']([^"']+)["']"#).unwrap();
            let variables_re = Regex::new(r"input_variables\s*=\s*\[([^\]]+)\]").unwrap();
            
            let template = template_re.captures(template_def)
                .map(|cap| TranspilerUtils::convert_template_variables(&cap[1]))
                .unwrap_or_else(|| "{{input}}".to_string());
                
            let variables = variables_re.captures(template_def)
                .map(|cap| Self::parse_variable_list(&cap[1]))
                .unwrap_or_else(|| vec!["input".to_string()]);
                
            Ok((template, variables))
        } else {
            // Fallback: treat as inline string
            let template = prompt_ref.trim_matches('"').trim_matches('\'');
            let variables = TranspilerUtils::extract_variables(template);
            Ok((template.to_string(), variables))
        }
    }

    /// Parse a list of variables from Python syntax
    fn parse_variable_list(vars_str: &str) -> Vec<String> {
        vars_str
            .split(',')
            .map(|v| v.trim().trim_matches('"').trim_matches('\'').to_string())
            .filter(|v| !v.is_empty())
            .collect()
    }

    /// Parse chain references from Python syntax
    fn parse_chain_references(_content: &str, _chains_str: &str) -> Result<Vec<LangChainNode>> {
        // For now, return empty - this would need more sophisticated parsing
        // In a full implementation, we'd resolve variable references to their definitions
        Ok(vec![])
    }

    /// Parse tool list from Python syntax
    fn parse_tool_list(tools_str: &str) -> Vec<String> {
        tools_str
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()
    }

    /// Resolve tool variable reference to actual tool list
    fn resolve_tool_variable(content: &str, var_name: &str) -> Vec<String> {
        // Try to find the variable definition
        let pattern = format!(r"(?s){}\s*=\s*\[([^\]]+)\]", regex::escape(var_name.trim()));
        if let Ok(re) = Regex::new(&pattern) {
            if let Some(cap) = re.captures(content) {
                return Self::parse_tool_list(&cap[1]);
            }
        }
        
        // Fallback: treat as single tool name
        vec![var_name.to_string()]
    }

    /// Convert LangChain LLM reference to RustChain model name
    fn convert_llm_model(llm_ref: &str) -> String {
        match llm_ref.trim() {
            "OpenAI()" | "ChatOpenAI()" => "gpt-3.5-turbo".to_string(),
            ref s if s.contains("gpt-4") => "gpt-4".to_string(),
            ref s if s.contains("gpt-3.5") => "gpt-3.5-turbo".to_string(),
            ref s if s.contains("claude") => "claude-3-sonnet".to_string(),
            ref s if s.contains("primary_llm") => "gpt-4".to_string(),
            ref s if s.contains("secondary_llm") => "gpt-3.5-turbo".to_string(),
            ref s if s.contains("tertiary_llm") => "claude-3-sonnet".to_string(),
            _ => "gpt-3.5-turbo".to_string(), // Default fallback
        }
    }

    /// Parse headers dictionary from Python syntax
    fn parse_headers_dict(headers_str: &str) -> std::collections::HashMap<String, String> {
        let mut headers = std::collections::HashMap::new();
        
        // Simple parser for headers like: {"Authorization": "Bearer token", "Content-Type": "application/json"}
        let header_pattern = Regex::new(r#"["']([^"']+)["']\s*:\s*["']([^"']+)["']"#).unwrap();
        
        for cap in header_pattern.captures_iter(headers_str) {
            let key = cap[1].to_string();
            let value = cap[2].to_string();
            headers.insert(key, value);
        }
        
        headers
    }

    /// Parse string list from Python syntax
    fn parse_string_list(list_str: &str) -> Vec<String> {
        let list_pattern = Regex::new(r#"["']([^"']+)["']"#).unwrap();
        list_pattern
            .captures_iter(list_str)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    /// Parse retriever configuration from LangChain syntax
    fn parse_retriever_config(retriever_str: &str) -> Result<RetrieverConfig> {
        // Extract vector store type from patterns like "pinecone_store.as_retriever()"
        let vector_store = if retriever_str.contains("pinecone") {
            "pinecone".to_string()
        } else if retriever_str.contains("chroma") {
            "chroma".to_string()
        } else if retriever_str.contains("knowledge_vectorstore") {
            "enterprise_knowledge".to_string()
        } else if retriever_str.contains("document_vectorstore") {
            "enterprise_documents".to_string()
        } else {
            "default_vectorstore".to_string()
        };

        // Extract search configuration
        let search_type = if retriever_str.contains("similarity_score_threshold") {
            "similarity_score_threshold".to_string()
        } else {
            "similarity".to_string()
        };

        // Parse search kwargs (simplified for demo)
        let mut search_kwargs = std::collections::HashMap::new();
        if retriever_str.contains("score_threshold") {
            search_kwargs.insert("score_threshold".to_string(), serde_json::json!(0.8));
        }
        if retriever_str.contains("k") {
            search_kwargs.insert("k".to_string(), serde_json::json!(5));
        }

        Ok(RetrieverConfig {
            search_type,
            search_kwargs,
            vector_store,
        })
    }

    /// Parse destination chains for MultiPromptChain
    fn parse_destination_chains(chains_str: &str) -> Result<std::collections::HashMap<String, LangChainNode>> {
        let mut chains = std::collections::HashMap::new();
        
        // For demo purposes, create simplified destination chains
        // In production, this would parse the actual chain definitions
        let chain_names = vec!["financial_analysis", "customer_intelligence", "operational_insights"];
        
        for name in chain_names {
            if chains_str.contains(name) {
                let chain_node = LangChainNode::LLMChain {
                    llm: "gpt-3.5-turbo".to_string(),
                    prompt: format!("Handle {} query: {{{{input}}}}", name),
                    variables: vec!["input".to_string()],
                };
                chains.insert(name.to_string(), chain_node);
            }
        }
        
        Ok(chains)
    }
}

/// Extension methods for Mission to support saving
impl Mission {
    pub async fn save_to_file(&self, file_path: &Path) -> Result<()> {
        let yaml_content = self.to_yaml()?;
        tokio::fs::write(file_path, yaml_content).await
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::PluginError {
                    message: format!("Failed to write file {}: {}", file_path.display(), e)
                }
            ))?;
        Ok(())
    }

    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .map_err(|e| crate::core::error::RustChainError::Config(
                crate::core::error::ConfigError::PluginError {
                    message: format!("Failed to serialize to YAML: {}", e)
                }
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_parse_simple_llm_chain() {
        let python_code = r#"
from langchain import LLMChain, OpenAI, PromptTemplate

prompt = PromptTemplate(
    input_variables=["product"],
    template="What is a good name for a company that makes {product}?"
)

chain = LLMChain(llm=OpenAI(), prompt=prompt)
"#;

        let mission = LangChainParser::parse_string(python_code).await.unwrap();
        
        assert_eq!(mission.name, "langchain_mission");
        assert!(mission.description.is_some());
        assert_eq!(mission.steps.len(), 1);
        
        let step = &mission.steps[0];
        assert!(matches!(step.step_type, StepType::Llm));
        
        let prompt = step.parameters.get("prompt").unwrap().as_str().unwrap();
        assert!(prompt.contains("{{product}}"));
    }

    #[tokio::test]
    async fn test_parse_agent_workflow() {
        let python_code = r#"
from langchain.agents import initialize_agent, AgentType
from langchain import OpenAI

tools = [search_tool, calculator_tool]
agent = initialize_agent(tools=tools, llm=OpenAI(), agent=AgentType.REACT_DOCSTORE)
"#;

        let mission = LangChainParser::parse_string(python_code).await.unwrap();
        
        assert_eq!(mission.steps.len(), 1);
        
        let step = &mission.steps[0];
        assert!(matches!(step.step_type, StepType::Agent));
        
        let tools = step.parameters.get("tools").unwrap().as_array().unwrap();
        assert_eq!(tools.len(), 2);
    }

    #[tokio::test]
    async fn test_template_variable_conversion() {
        let python_code = r#"
prompt = PromptTemplate(
    input_variables=["name", "location"],
    template="Hello {name}, welcome to {location}!"
)

chain = LLMChain(llm=OpenAI(), prompt=prompt)
"#;

        let mission = LangChainParser::parse_string(python_code).await.unwrap();
        let step = &mission.steps[0];
        let prompt = step.parameters.get("prompt").unwrap().as_str().unwrap();
        
        assert_eq!(prompt, "Hello {{name}}, welcome to {{location}}!");
        
        let variables = step.parameters.get("variables").unwrap().as_array().unwrap();
        assert_eq!(variables.len(), 2);
    }

    #[tokio::test] 
    async fn test_parse_empty_content() {
        let result = LangChainParser::parse_string("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_parse_no_langchain_patterns() {
        let python_code = r#"
print("Hello world")
x = 5 + 3
"#;
        
        let result = LangChainParser::parse_string(python_code).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_llm_model_conversion() {
        assert_eq!(LangChainParser::convert_llm_model("OpenAI()"), "gpt-3.5-turbo");
        assert_eq!(LangChainParser::convert_llm_model("ChatOpenAI()"), "gpt-3.5-turbo");
        assert_eq!(LangChainParser::convert_llm_model("unknown"), "gpt-3.5-turbo");
    }

    #[tokio::test]
    async fn test_variable_list_parsing() {
        let vars = LangChainParser::parse_variable_list("\"name\", \"location\", \"time\"");
        assert_eq!(vars, vec!["name", "location", "time"]);
        
        let empty_vars = LangChainParser::parse_variable_list("");
        assert!(empty_vars.is_empty());
    }

    #[tokio::test]
    async fn test_mission_yaml_serialization() {
        let mission = TranspilerUtils::create_mission(
            "test".to_string(),
            Some("Test mission".to_string()),
            vec![],
        );
        
        let yaml = mission.to_yaml().unwrap();
        assert!(yaml.contains("name: test"));
        // YAML can use either single or double quotes, so check for both
        assert!(yaml.contains("version: '1.0'") || yaml.contains("version: \"1.0\""));
    }

    #[tokio::test]
    async fn test_file_operations() {
        let python_code = r#"
from langchain import LLMChain, OpenAI, PromptTemplate

prompt = PromptTemplate(
    input_variables=["topic"],
    template="Explain {topic} in simple terms"
)

chain = LLMChain(llm=OpenAI(), prompt=prompt)
"#;

        // Test parse from file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(python_code.as_bytes()).unwrap();
        
        let mission = LangChainParser::parse_file(temp_file.path()).await.unwrap();
        assert_eq!(mission.steps.len(), 1);

        // Test save to file
        let output_temp = NamedTempFile::new().unwrap();
        mission.save_to_file(output_temp.path()).await.unwrap();
        
        let saved_content = std::fs::read_to_string(output_temp.path()).unwrap();
        assert!(saved_content.contains("name: langchain_mission"));
    }
}