use crate::agent::serialization::Strategy;
use clap::Parser;

/// Get things done with LLMs.
#[derive(Parser, Debug, Clone, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Generator string as <type>://<model name>@<host>:<port>
    #[arg(short = 'G', long, default_value = "ollama://llama3@localhost:11434")]
    pub generator: String,
    /// Judge generator string as <type>://<model name>@<host>:<port>
    #[arg(short = 'J', long, default_value = "ollama://llama3@localhost:11434")]
    pub judge: String,
    /// Run in judge mode.
    #[arg(long)]
    pub judge_mode: bool,
    /// Only rely on user prompt. Use for models like openai/o1 family that don't allow a system prompt.
    #[arg(long)]
    pub user_only: bool,
    /// Embedder string as <type>://<model name>@<host>:<port>
    #[arg(
        short = 'E',
        long,
        default_value = "ollama://all-minilm@localhost:11434"
    )]
    pub embedder: String,
    /// Serialization strategy.
    #[arg(short = 'S', long, default_value_t, value_enum)]
    pub serialization: Strategy,
    /// Conversation window, it can be either "full" (full chat history), "summary" (report the last messages entirely and compress the previous ones) or "N" (last N messages).
    #[arg(long, default_value = "15")]
    pub window: String,
    /// Force specified serialization format even if the model supports native tools calling.
    #[arg(long)]
    pub force_format: bool,
    /// Chain-of-Thought xml tags.
    #[arg(long, value_parser, num_args = 1.., value_delimiter = ' ', default_values_t = ["think".to_string(), "thinking".to_string(), "reasoning".to_string()])]
    pub cot_tags: Vec<String>,
    /// Tasklet file.
    #[arg(short = 'T', long)]
    pub tasklet: Option<String>,
    /// Workflow file.
    #[arg(short = 'W', long)]
    pub workflow: Option<String>,
    /// Specify the prompt if not provided by the tasklet.
    #[arg(short = 'P', long)]
    pub prompt: Option<String>,
    /// Pre define variables.
    #[arg(short = 'D', long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub define: Vec<String>,
    /// Robopages server address. If specified, the agent will use the tools defined by the robopages server.
    #[arg(short = 'R', long)]
    pub robopages: Option<String>,
    /// Context window size.
    #[arg(long, default_value_t = 8000)]
    pub context_window: u32,
    /// Maximum number of steps to complete the task or 0 for no limit.
    #[arg(long, default_value_t = 0)]
    pub max_iterations: usize,
    /// If set, the agent will sleep for the given number of seconds between each step.
    #[arg(long)]
    pub sleep: Option<usize>,
    /// Record every event of the session to a JSONL file.
    #[arg(long)]
    pub record_to: Option<String>,
    /// If set, the agent will start a web UI on the given port.
    #[arg(short = 'U', long)]
    pub web_ui: bool,

    /// Web UI address.
    #[arg(long, default_value = "127.0.0.1:8080")]
    pub web_address: String,
    /// Websocket server address for the web UI.
    #[arg(long, default_value = "127.0.0.1:8081")]
    pub ws_address: String,

    /// Print the documentation of the available tool namespaces.
    #[arg(long)]
    pub generate_doc: bool,
}
