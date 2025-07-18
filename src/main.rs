//! llm-spatial-tester – compare several LLMs on geometry prompts
use std::{fs, io::Write, sync::Arc};

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use clap::Parser;
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
    client: async_openai::Client<async_openai::config::OpenAIConfig>,
}
impl OpenAi {
    fn new(api_key: Option<String>) -> Result<Self> {
        // fall back to env var if no flag was provided
        let cfg = match api_key {
            Some(k) => async_openai::config::OpenAIConfig::new().with_api_key(k),
            None => async_openai::config::OpenAIConfig::default(),
        };
        Ok(Self {
            client: async_openai::Client::with_config(cfg),
        })
    }
}
#[async_trait]
impl LlmClient for OpenAi {
    fn name(&self) -> &str {
        "OpenAI o3"
    }

    async fn ask(&self, prompt: &str) -> Result<String> {
        // ::chat().create_byot takes arbitrary JSON thanks to the `byot` feature
        let req = serde_json::json!({
            "model": "o3",
            "messages": [
                { "role":async_openai::types::Role::User, "content": prompt }
            ],
            "response_format": { "type": "text" },
            "reasoning_effort": "medium"
        });

        let resp: serde_json::Value = self.client.chat().create_byot(req).await?;

        Ok(resp["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("<no content>")
            .to_owned())
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
            model: "claude-opus-4-20250514",
            messages: vec![Msg {
                role: "user",
                content: prompt,
            }],
            max_tokens: 1024,
        };

        let text = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", key)
            .header("anthropic-version", "2023-06-01")
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let resp: Resp =
            serde_json::from_str(&text).map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err))?;

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

        #[derive(Serialize, Deserialize)]
        struct Text {
            text: String,
        }
        #[derive(Serialize, Deserialize)]
        struct Part {
            parts: Vec<Text>,
        }
        #[derive(Serialize)]
        struct Req {
            contents: Vec<Part>,
        }
        #[derive(Deserialize)]
        struct Resp {
            candidates: Vec<Cand>,
        }
        #[derive(Deserialize)]
        struct Cand {
            content: Part,
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-preview-05-06:generateContent?key={}",
            key
        );
        let req = Req {
            contents: vec![Part {
                parts: vec![Text {
                    text: prompt.to_string(),
                }],
            }],
        };

        let text = self
            .http
            .post(url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let resp: Resp =
            serde_json::from_str(&text).map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err))?;

        Ok(resp
            .candidates
            .first()
            .map(|c| {
                c.content
                    .parts
                    .iter()
                    .map(|p| p.text.as_str())
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .unwrap_or_default())
    }
}

/*────────────────────── DeepSeek R1 (stub) ───────────────────────*/
struct DeepSeek {
    http: Arc<Client>,
    key: Option<String>,
}

impl DeepSeek {
    pub fn new(http: Arc<Client>, key: Option<String>) -> Self {
        Self { http, key }
    }
}

#[async_trait]
impl LlmClient for DeepSeek {
    fn name(&self) -> &str {
        "DeepSeek Reasoner"
    }

    async fn ask(&self, prompt: &str) -> Result<String> {
        let key = self
            .key
            .as_deref()
            .ok_or_else(|| anyhow!("DEEPSEEK_API_KEY not provided"))?;

        /* ---- request payload: OpenAI-compatible schema ---- */
        #[derive(Serialize)]
        struct Msg<'a> {
            role: &'static str,
            content: &'a str,
        }
        #[derive(Serialize)]
        struct Req<'a> {
            model: &'static str,
            messages: Vec<Msg<'a>>,
        }

        let req = Req {
            model: "deepseek-reasoner", // reasoning model R1
            messages: vec![Msg {
                role: "user",
                content: prompt,
            }],
        };

        /* ---- fire & parse ---- */
        #[derive(Deserialize)]
        struct Choice {
            message: MsgOut,
        }
        #[derive(Deserialize)]
        struct MsgOut {
            content: String,
        }
        #[derive(Deserialize)]
        struct Resp {
            choices: Vec<Choice>,
        }

        let text = self
            .http
            .post("https://api.deepseek.com/chat/completions")
            .bearer_auth(key)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let resp: Resp =
            serde_json::from_str(&text).map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err))?;

        Ok(resp
            .choices
            .first()
            .map(|c| c.message.content.trim().to_owned())
            .unwrap_or_else(|| "<no content>".into()))
    }
}
/*────────────────────────── Grok (stub) ──────────────────────────*/
struct Grok {
    http: Arc<Client>,
    key: Option<String>,
}

impl Grok {
    pub fn new(http: Arc<Client>, key: Option<String>) -> Self {
        Self { http, key }
    }
}

#[async_trait]
impl LlmClient for Grok {
    fn name(&self) -> &str {
        "Grok 3"
    }

    async fn ask(&self, prompt: &str) -> Result<String> {
        let key = self.key.as_deref().ok_or_else(|| anyhow!("XAI_API_KEY not provided"))?;

        /* --- request payload mirrors the official example --- */
        #[derive(Serialize)]
        struct Msg<'a> {
            role: &'static str,
            content: &'a str,
        }

        #[derive(Serialize)]
        struct Req<'a> {
            model: &'static str,
            messages: Vec<Msg<'a>>,
            stream: bool,
            temperature: f32,
        }

        let req = Req {
            model: "grok-3-latest", // public Grok-3 slug
            messages: vec![Msg {
                role: "user",
                content: prompt,
            }],
            stream: false,
            temperature: 0.7,
        };

        /* --- fire & parse --- */
        #[derive(Deserialize)]
        struct Choice {
            message: MsgOut,
        }
        #[derive(Deserialize)]
        struct MsgOut {
            content: String,
        }
        #[derive(Deserialize)]
        struct Resp {
            choices: Vec<Choice>,
        }

        let text = self
            .http
            .post("https://api.x.ai/v1/chat/completions")
            .bearer_auth(key)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let resp: Resp =
            serde_json::from_str(&text).map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err))?;

        Ok(resp
            .choices
            .first()
            .map(|c| c.message.content.trim().to_owned())
            .unwrap_or_else(|| "<no content>".into()))
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

    // Make sure the output directory exists.
    let out_dir = std::path::Path::new("results");
    fs::create_dir_all(out_dir)?;

    /* 3 ▸ shared HTTP client + provider roster */
    let http = Arc::new(Client::builder().build()?);
    let providers: Vec<Box<dyn LlmClient>> = vec![
        Box::new(OpenAi::new(cli.openai_key.clone())?),
        Box::new(Claude::new(http.clone(), cli.anthropic_key.clone())),
        Box::new(Gemini::new(http.clone(), cli.google_key.clone())),
        Box::new(DeepSeek::new(http.clone(), cli.deepseek_key.clone())),
        Box::new(Grok::new(http.clone(), cli.grok_key.clone())),
    ];

    /* 4 ▸ ask every model every prompt (concurrently) */
    for (idx, prompt) in prompts.iter().enumerate() {
        // Skip the prompt if we already have a result file for it.
        let file_path = out_dir.join(format!("prompt_{:02}.md", idx + 1));
        if file_path.exists() {
            println!(
                "Skipping prompt #{}: already exists at {}",
                idx + 1,
                file_path.display()
            );
            continue;
        }
        println!("\n════════════════════════════════════════════════════");
        println!("PROMPT #{}:\n{}\n", idx + 1, prompt);

        let mut futs = Vec::new();
        for p in &providers {
            let name = p.name().to_string();
            let fut = async move {
                let start = std::time::Instant::now();
                let res = p.ask(prompt).await;
                let t = start.elapsed();
                (name, res, t)
            };
            futs.push(fut);
        }

        // collect into Vec<(name, result)>
        let results = join_all(futs).await;

        // ----- print to console & build a single text blob -----
        let mut file_buf = format!("# Prompt #{}: {}\n\n", idx + 1, prompt);
        for (name, reply, t) in &results {
            match reply {
                Ok(text) => {
                    println!(
                        "── {}  ({} secs) ──────────────────────────────────────",
                        name,
                        t.as_secs()
                    );
                    println!("{}\n", text.trim());
                    file_buf.push_str(&format!("## {name}  ({} secs)\n{text}\n\n", t.as_secs()));
                }
                Err(e) => {
                    eprintln!("!! {}: {}\n", name, e);
                    file_buf.push_str(&format!("## {name}  ({} secs)\nERROR: {e}\n\n", t.as_secs()));
                }
            }
        }

        let mut f = fs::File::create(&file_path)?;
        f.write_all(file_buf.as_bytes())?;
        println!("📄  saved to {}\n", file_path.display());
    }
    Ok(())
}
