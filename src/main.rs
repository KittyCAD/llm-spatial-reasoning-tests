//! llm-spatial-tester – compare several LLMs on geometry prompts
//!
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use clap::Parser;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs, sync::Arc};

/*───────────────────────────── CLI ────────────────────────────────*/
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to TOML file containing `prompts = [ ... ]`
    #[arg(short, long, default_value = "prompts.toml", env = "PROMPTS_FILE")]
    prompts: String,

    // Each key can come from a flag *or* its matching environment variable.
    #[arg(long, env = "OPENAI_API_KEY")]
    openai_key: Option<String>,

    #[arg(long, env = "ANTHROPIC_API_KEY")]
    anthropic_key: Option<String>,

    #[arg(long, env = "GOOGLE_API_KEY")]
    google_key: Option<String>,

    #[arg(long, env = "DEEPSEEK_API_KEY")]
    deepseek_key: Option<String>,

    #[arg(long, env = "GROK_API_KEY")]
    grok_key: Option<String>,
}

/*──────────────────── Provider trait (async) ──────────────────────*/
#[async_trait]
trait LlmClient: Send + Sync {
    fn name(&self) -> &str;
    async fn ask(&self, prompt: &str) -> Result<String>;
}

/*────────────────────────── OpenAI o3 ─────────────────────────────*/
struct OpenAi {
    http: Arc<Client>,
    key: Option<String>,
}
impl OpenAi {
    fn new(http: Arc<Client>, key: Option<String>) -> Self {
        Self { http, key }
    }
}
#[async_trait]
impl LlmClient for OpenAi {
    fn name(&self) -> &str {
        "OpenAI o3"
    }

    async fn ask(&self, prompt: &str) -> Result<String> {
        let key = self
            .key
            .as_deref()
            .ok_or_else(|| anyhow!("OPENAI_API_KEY not provided"))?;

        #[derive(Serialize)]
        struct Msg<'m> {
            role: &'m str,
            content: &'m str,
        }
        #[derive(Serialize)]
        struct Req<'r> {
            model: &'r str,
            messages: Vec<Msg<'r>>,
            max_tokens: u16,
        }
        #[derive(Deserialize)]
        struct Resp {
            choices: Vec<Choice>,
        }
        #[derive(Deserialize)]
        struct Choice {
            message: MsgOut,
        }
        #[derive(Deserialize)]
        struct MsgOut {
            content: String,
        }

        let req = Req {
            model: "gpt-4o-mini",
            messages: vec![Msg {
                role: "user",
                content: prompt,
            }],
            max_tokens: 1024,
        };

        let resp: Resp = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(key)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }
}

/*────────────────────── Anthropic Claude 4 ───────────────────────*/
struct Claude {
    http: Arc<Client>,
    key: Option<String>,
}
impl Claude {
    fn new(http: Arc<Client>, key: Option<String>) -> Self {
        Self { http, key }
    }
}
#[async_trait]
impl LlmClient for Claude {
    fn name(&self) -> &str {
        "Claude 4 Sonnet"
    }

    async fn ask(&self, prompt: &str) -> Result<String> {
        let key = self
            .key
            .as_deref()
            .ok_or_else(|| anyhow!("ANTHROPIC_API_KEY not provided"))?;

        #[derive(Serialize)]
        struct Msg<'m> {
            role: &'m str,
            content: &'m str,
        }
        #[derive(Serialize)]
        struct Req<'r> {
            model: &'r str,
            messages: Vec<Msg<'r>>,
            max_tokens: u16,
        }
        #[derive(Deserialize)]
        struct Resp {
            content: Vec<Seg>,
        }
        #[derive(Deserialize)]
        struct Seg {
            text: String,
        }

        let req = Req {
            model: "claude-4-sonnet-20240229",
            messages: vec![Msg {
                role: "user",
                content: prompt,
            }],
            max_tokens: 1024,
        };

        let resp: Resp = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let answer = resp
            .content
            .iter()
            .map(|seg| seg.text.as_str())
            .collect::<Vec<&str>>()
            .join("");
        Ok(answer)
    }
}

/*──────────────────────── Google Gemini ──────────────────────────*/
struct Gemini {
    http: Arc<Client>,
    key: Option<String>,
}
impl Gemini {
    fn new(http: Arc<Client>, key: Option<String>) -> Self {
        Self { http, key }
    }
}
#[async_trait]
impl LlmClient for Gemini {
    fn name(&self) -> &str {
        "Gemini Pro"
    }

    async fn ask(&self, prompt: &str) -> Result<String> {
        let key = self
            .key
            .as_deref()
            .ok_or_else(|| anyhow!("GOOGLE_API_KEY not provided"))?;

        #[derive(Serialize)]
        struct Part<'p> {
            role: &'p str,
            text: &'p str,
        }
        #[derive(Serialize)]
        struct Req<'r> {
            contents: Vec<Part<'r>>,
        }
        #[derive(Deserialize)]
        struct Resp {
            candidates: Vec<Cand>,
        }
        #[derive(Deserialize)]
        struct Cand {
            content: Vec<Seg>,
        }
        #[derive(Deserialize)]
        struct Seg {
            text: String,
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
            key
        );
        let req = Req {
            contents: vec![Part {
                role: "user",
                text: prompt,
            }],
        };

        let resp: Resp = self
            .http
            .post(url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp
            .candidates
            .first()
            .map(|c| {
                c.content
                    .iter()
                    .map(|seg| seg.text.as_str())
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .unwrap_or_default())
    }
}

/*────────────────────── DeepSeek R1 (stub) ───────────────────────*/
struct DeepSeek {
    key_present: bool,
}
impl DeepSeek {
    fn new(key: Option<String>) -> Self {
        Self {
            key_present: key.is_some(),
        }
    }
}
#[async_trait]
impl LlmClient for DeepSeek {
    fn name(&self) -> &str {
        "DeepSeek R1"
    }
    async fn ask(&self, _prompt: &str) -> Result<String> {
        if self.key_present {
            Err(anyhow!("DeepSeek R1 endpoint TBD"))
        } else {
            Err(anyhow!("DEEPSEEK_API_KEY not provided"))
        }
    }
}

/*────────────────────────── Grok (stub) ──────────────────────────*/
struct Grok {
    key_present: bool,
}
impl Grok {
    fn new(key: Option<String>) -> Self {
        Self {
            key_present: key.is_some(),
        }
    }
}
#[async_trait]
impl LlmClient for Grok {
    fn name(&self) -> &str {
        "Grok"
    }
    async fn ask(&self, _prompt: &str) -> Result<String> {
        if self.key_present {
            Err(anyhow!("Grok endpoint TBD"))
        } else {
            Err(anyhow!("GROK_API_KEY not provided"))
        }
    }
}

/*────────────────────────────── main ─────────────────────────────*/
#[tokio::main]
async fn main() -> Result<()> {
    /* 1 ▸ parse CLI; env-driven defaults handled by clap */
    let cli = Cli::parse();

    /* 2 ▸ load prompts list */
    let raw = fs::read_to_string(&cli.prompts).map_err(|e| anyhow!("failed to read {}: {e}", &cli.prompts))?;
    #[derive(Deserialize)]
    struct P {
        prompts: Vec<String>,
    }
    let P { prompts } = toml::from_str(&raw).map_err(|e| anyhow!("{} parse error: {e}", &cli.prompts))?;
    if prompts.is_empty() {
        return Err(anyhow!("{} contains no prompts", &cli.prompts));
    }

    /* 3 ▸ shared HTTP client + provider roster */
    let http = Arc::new(Client::builder().build()?);
    let providers: Vec<Box<dyn LlmClient>> = vec![
        Box::new(OpenAi::new(http.clone(), cli.openai_key.clone())),
        Box::new(Claude::new(http.clone(), cli.anthropic_key.clone())),
        Box::new(Gemini::new(http.clone(), cli.google_key.clone())),
        Box::new(DeepSeek::new(cli.deepseek_key.clone())),
        Box::new(Grok::new(cli.grok_key.clone())),
    ];

    /* 4 ▸ ask every model every prompt (concurrently) */
    for (idx, prompt) in prompts.iter().enumerate() {
        println!("\n════════════════════════════════════════════════════");
        println!("PROMPT #{}:\n{}\n", idx + 1, prompt);

        let mut futs = Vec::new();
        for p in &providers {
            let name = p.name().to_string();
            let fut = async move { (name, p.ask(prompt).await) };
            futs.push(fut);
        }

        for (name, reply) in join_all(futs).await {
            match reply {
                Ok(text) => {
                    println!("── {} ──────────────────────────────────────", name);
                    println!("{}\n", text.trim());
                }
                Err(e) => eprintln!("!! {}: {}\n", name, e),
            }
        }
    }
    Ok(())
}
