use anyhow::{Result, anyhow};
use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let toml_raw = fs::read_to_string("prompts.toml")
        .map_err(|e| anyhow!("failed to read prompts.toml: {}", e))?;
    #[derive(Deserialize)]
    struct PromptFile {
        prompts: Vec<String>,
    }
    let PromptFile { prompts } =
        toml::from_str(&toml_raw).map_err(|e| anyhow!("prompts.toml parse error: {}", e))?;
    if prompts.is_empty() {
        return Err(anyhow!("prompts.toml contains no prompts"));
    }

    // ── 1.  Spatial-reasoning prompts you want to compare ────────────────
    let prompts = vec![
        "Object A is 24 mm horizontally and 46 mm vertically away; \
         Object B is 6 in horizontally and 8 in vertically away. Which is closer, \
         and by how much?",
        "Slice a cube with a plane that passes through the mid-points \
         of three mutually adjacent edges.  What is the cross-section shape?",
        "Fold a square sheet along its vertical center line. Punch a single hole \
         1 cm from the top-left corner of the folded sheet. Unfold it—where are \
         the holes relative to the original corners?",
    ];

    // ── 2.  Spin up reqwest once and share it ────────────────────────────
    let http = Client::builder().build()?;

    // ── 3.  Instantiate all providers (boxed trait objects) ──────────────
    let providers: Vec<Box<dyn LlmClient>> = vec![
        Box::new(OpenAi::new(&http)?),
        Box::new(Claude::new(&http)?),
        Box::new(Gemini::new(&http)?),
        Box::new(DeepSeek::new(&http)?),
        Box::new(Grok::new(&http)?),
    ];

    // ── 4.  Ask every model every prompt (concurrently) ──────────────────
    for (i, prompt) in prompts.iter().enumerate() {
        println!("\n════════════════════════════════════════════════════");
        println!("PROMPT #{}:\n{}\n", i + 1, prompt);

        // one async request per provider
        let futures = providers
            .iter()
            .map(|p| p.ask(prompt).map(|r| (p.name(), r)));
        for (name, reply) in join_all(futures).await {
            match reply {
                Ok(text) => {
                    println!("── {} ─────────────────────────────────────", name);
                    println!("{}\n", text.trim());
                }
                Err(e) => {
                    eprintln!("!! {} failed: {}\n", name, e);
                }
            }
        }
    }

    Ok(())
}

trait LlmClient: Send + Sync {
    fn name(&self) -> &str;
    fn ask<'a>(&'a self, prompt: &'a str) -> futures::future::BoxFuture<'a, Result<String>>;
}

/*──────────────────────────── OpenAI (o3) ────────────────────────────*/
struct OpenAi<'a> {
    http: &'a Client,
    key: String,
}
impl<'a> OpenAi<'a> {
    fn new(http: &'a Client) -> Result<Self> {
        Ok(Self {
            http,
            key: env::var("OPENAI_API_KEY").map_err(|_| anyhow!("OPENAI_API_KEY not set"))?,
        })
    }
}
impl<'a> LlmClient for OpenAi<'a> {
    fn name(&self) -> &str {
        "OpenAI o3"
    }
    fn ask<'b>(&'b self, prompt: &'b str) -> futures::future::BoxFuture<'b, Result<String>> {
        Box::pin(async move {
            #[derive(Serialize)]
            struct Req<'r> {
                model: &'r str,
                messages: Vec<Msg<'r>>,
                max_tokens: u16,
            }
            #[derive(Serialize)]
            struct Msg<'m> {
                role: &'m str,
                content: &'m str,
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
                model: "gpt-4o-mini", // adjust if you have full-fat 4o
                messages: vec![Msg {
                    role: "user",
                    content: prompt,
                }],
                max_tokens: 1024,
            };
            let resp: Resp = self
                .http
                .post("https://api.openai.com/v1/chat/completions")
                .bearer_auth(&self.key)
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
        })
    }
}

/*────────────────────────── Anthropic Claude ─────────────────────────*/
struct Claude<'a> {
    http: &'a Client,
    key: String,
}
impl<'a> Claude<'a> {
    fn new(http: &'a Client) -> Result<Self> {
        Ok(Self {
            http,
            key: env::var("ANTHROPIC_API_KEY").map_err(|_| anyhow!("ANTHROPIC_API_KEY not set"))?,
        })
    }
}
impl<'a> LlmClient for Claude<'a> {
    fn name(&self) -> &str {
        "Claude 4 Sonnet"
    }
    fn ask<'b>(&'b self, prompt: &'b str) -> futures::future::BoxFuture<'b, Result<String>> {
        Box::pin(async move {
            #[derive(Serialize)]
            struct Req<'r> {
                model: &'r str,
                messages: Vec<Msg<'r>>,
                max_tokens: u16,
            }
            #[derive(Serialize)]
            struct Msg<'m> {
                role: &'m str,
                content: &'m str,
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
                .header("x-api-key", &self.key)
                .header("anthropic-version", "2023-06-01")
                .json(&req)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            Ok(resp
                .content
                .iter()
                .map(|c| &c.text)
                .collect::<Vec<_>>()
                .join(""))
        })
    }
}

/*──────────────────────────── Google Gemini ──────────────────────────*/
struct Gemini<'a> {
    http: &'a Client,
    key: String,
}
impl<'a> Gemini<'a> {
    fn new(http: &'a Client) -> Result<Self> {
        Ok(Self {
            http,
            key: env::var("GOOGLE_API_KEY").map_err(|_| anyhow!("GOOGLE_API_KEY not set"))?,
        })
    }
}
impl<'a> LlmClient for Gemini<'a> {
    fn name(&self) -> &str {
        "Gemini Pro"
    }
    fn ask<'b>(&'b self, prompt: &'b str) -> futures::future::BoxFuture<'b, Result<String>> {
        Box::pin(async move {
            #[derive(Serialize)]
            struct Req<'r> {
                contents: Vec<Part<'r>>,
            }
            #[derive(Serialize)]
            struct Part<'p> {
                role: &'p str,
                text: &'p str,
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
                self.key
            );
            let req = Req {
                contents: vec![Part {
                    role: "user",
                    text: prompt,
                }],
            };
            let resp: Resp = self
                .http
                .post(&url)
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
                        .map(|s| &s.text)
                        .collect::<Vec<_>>()
                        .join("")
                })
                .unwrap_or_default())
        })
    }
}

/*────────────────────────── DeepSeek R1 (beta) ───────────────────────*/
struct DeepSeek<'a> {
    http: &'a Client,
    key: Option<String>,
}
impl<'a> DeepSeek<'a> {
    fn new(http: &'a Client) -> Result<Self> {
        Ok(Self {
            http,
            key: env::var("DEEPSEEK_API_KEY").ok(), // optional until public
        })
    }
}
impl<'a> LlmClient for DeepSeek<'a> {
    fn name(&self) -> &str {
        "DeepSeek R1"
    }
    fn ask<'b>(&'b self, prompt: &'b str) -> futures::future::BoxFuture<'b, Result<String>> {
        Box::pin(async move {
            if self.key.is_none() {
                return Ok("[no API key / endpoint yet]".into());
            }
            // TODO: update with real endpoint once public
            Err(anyhow!("DeepSeek R1 endpoint TBD"))
        })
    }
}

/*──────────────────────────── xAI Grok (beta) ────────────────────────*/
struct Grok<'a> {
    http: &'a Client,
    key: Option<String>,
}
impl<'a> Grok<'a> {
    fn new(http: &'a Client) -> Result<Self> {
        Ok(Self {
            http,
            key: env::var("GROK_API_KEY").ok(), // optional
        })
    }
}
impl<'a> LlmClient for Grok<'a> {
    fn name(&self) -> &str {
        "Grok"
    }
    fn ask<'b>(&'b self, prompt: &'b str) -> futures::future::BoxFuture<'b, Result<String>> {
        Box::pin(async move {
            if self.key.is_none() {
                return Ok("[no API key / endpoint yet]".into());
            }
            // TODO: update with real endpoint once public
            Err(anyhow!("Grok endpoint TBD"))
        })
    }
}
