use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command;
use which::which;

#[derive(Clone)]
struct AiTool {
    name: &'static str,
    command: &'static str,
    description: &'static str,
}

fn get_tools() -> Vec<AiTool> {
    vec![
        AiTool {
            name: "OpenCode",
            command: "opencode",
            description: "Open source AI coding agent",
        },
        AiTool {
            name: "Kilo",
            command: "kilo",
            description: "Interactive CLI coding assistant",
        },
        AiTool {
            name: "Cline",
            command: "cline",
            description: "Autonomous coding agent",
        },
        AiTool {
            name: "Gemini CLI",
            command: "gemini",
            description: "Google Gemini AI assistant",
        },
        AiTool {
            name: "Qwen",
            command: "qwen",
            description: "Alibaba Qwen AI assistant",
        },
        AiTool {
            name: "Claude CLI",
            command: "claude",
            description: "Anthropic Claude AI assistant",
        },
        AiTool {
            name: "Aider",
            command: "aider",
            description: "AI pair programming in your terminal",
        },
        AiTool {
            name: "Cursor",
            command: "cursor",
            description: "AI-powered code editor",
        },
        AiTool {
            name: "Continue",
            command: "continue",
            description: "Open source AI code assistant",
        },
        AiTool {
            name: "Cody",
            command: "cody",
            description: "Sourcegraph AI coding assistant",
        },
        AiTool {
            name: "Copilot CLI",
            command: "github-copilot-cli",
            description: "GitHub Copilot for terminal",
        },
        AiTool {
            name: "Ollama",
            command: "ollama",
            description: "Run LLMs locally",
        },
        AiTool {
            name: "LM Studio",
            command: "lmstudio",
            description: "Local LLM runner",
        },
        AiTool {
            name: "Mistral",
            command: "mistral",
            description: "Mistral AI assistant",
        },
        AiTool {
            name: "GPT CLI",
            command: "chatgpt",
            description: "ChatGPT CLI interface",
        },
    ]
}

fn check_installed(command: &str) -> bool {
    which(command).is_ok()
}

fn run_tool(command: &str) -> Result<()> {
    let mut child = Command::new(command)
        .env("TERM", "xterm-256color")
        .spawn()?;

    child.wait()?;
    Ok(())
}

fn print_banner() {
    println!();
    println!(
        "{}",
        "  ██████╗██╗██╗    ████████╗███████╗ ██████╗".cyan().bold()
    );
    println!(
        "{}",
        " ██╔════╝██║██║    ╚══██╔══╝██╔═══██╗██╔══██╗"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        " ██║     ██║██║       ██║   ██║   ██║██████╔╝"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        " ██║     ██║██║       ██║   ██║   ██║██╔══██╗"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        " ╚██████╗██║██║       ██║   ╚██████╔╝██║  ██║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "  ╚═════╝╚═╝╚═╝       ╚═╝    ╚═════╝ ╚═╝  ╚═╝"
            .cyan()
            .bold()
    );
    println!();
    println!(
        "{}",
        "    AI Agent Launcher - Select your AI tool"
            .white()
            .dimmed()
    );
    println!();
}

fn main() -> Result<()> {
    print_banner();

    let tools = get_tools();
    let installed: Vec<AiTool> = tools
        .into_iter()
        .filter(|t| check_installed(t.command))
        .collect();

    if installed.is_empty() {
        println!("{}", "  No AI tools found on your system.".yellow());
        println!();
        println!("  Install one of the following tools to get started:");
        for tool in get_tools() {
            println!("    • {} - {}", tool.name.white().bold(), tool.description);
        }
        println!();
        println!("  Check your PATH or install an AI tool first.");
        println!();
        return Ok(());
    }

    let items: Vec<String> = installed
        .iter()
        .map(|tool| format!("  {} - {}", tool.name.white().bold(), tool.description))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select AI tool to launch")
        .default(0)
        .items(&items)
        .interact()?;

    let tool = &installed[selection];

    println!("\n  Launching {}...\n", tool.name.cyan().bold());

    run_tool(tool.command)?;

    println!("\n  Returned from {}.\n", tool.name.cyan());

    Ok(())
}
