use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderParams {
    #[serde(default)]
    pub ollama: OllamaParams,
    #[serde(default)]
    pub openai: OpenAIParams,
    #[serde(default)]
    pub google: GoogleParams,
    #[serde(default)]
    pub lmstudio: LMStudioParams,
    #[serde(default)]
    pub custom: CustomParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaParams {
    pub url: String,
    pub model: String,
}

impl Default for OllamaParams {
    fn default() -> Self {
        Self {
            url: "http://localhost:11434".to_string(),
            model: "gemma2:2b".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIParams {
    pub api_key: String,
    #[serde(default, rename = "web-search-enabled")]
    pub web_search_enabled: bool,
}

impl Default for OpenAIParams {
    fn default() -> Self {
        Self {
            api_key: "__NOT_STORED_HERE__".to_string(),
            web_search_enabled: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleParams {
    pub api_key: String,
}

impl Default for GoogleParams {
    fn default() -> Self {
        Self {
            api_key: "__NOT_STORED_HERE__".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LMStudioParams {
    pub url: String,
    pub model: String,
}

impl Default for LMStudioParams {
    fn default() -> Self {
        Self {
            url: "http://localhost:1234".to_string(),
            model: "local-model".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomParams {
    pub url: String,
}

impl Default for CustomParams {
    fn default() -> Self {
        Self {
            url: String::new(),
        }
    }
}

impl Default for ProviderParams {
    fn default() -> Self {
        Self {
            ollama: OllamaParams::default(),
            openai: OpenAIParams::default(),
            google: GoogleParams::default(),
            lmstudio: LMStudioParams::default(),
            custom: CustomParams::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub provider: String,
    #[serde(rename = "provider-params")]
    pub provider_params: ProviderParams,
    #[serde(rename = "conversation-history")]
    pub conversation_history: bool,
    #[serde(rename = "keyboard-shortcut", default = "default_shortcut")]
    pub keyboard_shortcut: String,
}

pub fn default_shortcut() -> String {
    "Ctrl+Space".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            provider_params: ProviderParams::default(),
            conversation_history: true,
            keyboard_shortcut: default_shortcut(),
        }
    }
}

