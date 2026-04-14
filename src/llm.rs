use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::models::{ContentAnalysis, ReverseSearch, Video};

#[derive(Debug, Serialize)]
struct LLMRequest {
    model: String,
    messages: Vec<LLMMessage>,
    temperature: f64,
}

#[derive(Debug, Serialize)]
struct LLMMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct LLMResponse {
    choices: Vec<LLMChoice>,
}

#[derive(Debug, Deserialize)]
struct LLMChoice {
    message: LLMMessage,
}

pub struct LLMClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl LLMClient {
    pub fn new(api_key: String) -> Self {
        LLMClient {
            client: Client::new(),
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
        }
    }

    pub fn with_custom_endpoint(mut self, base_url: String, model: String) -> Self {
        self.base_url = base_url;
        self.model = model;
        self
    }

    async fn send_request(&self, messages: Vec<LLMMessage>) -> Result<String> {
        let request = LLMRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("LLM API error: {}", error_text);
        }

        let llm_response: LLMResponse = response.json().await?;
        
        Ok(llm_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from LLM"))?
            .message
            .content
            .clone())
    }

    pub async fn analyze_content(&self, videos: &[Video]) -> Result<ContentAnalysis> {
        let content_summary = videos
            .iter()
            .map(|v| format!("标题: {}, 作者: {}", v.title, v.author))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "请分析以下Bilibili推荐视频内容，识别信息茧房倾向性：

视频列表：
{}

请提供：
1. 内容总结（100字以内）
2. 主要话题标签（5-10个）
3. 偏见分析（指出可能的倾向性）
4. 信息茧房评分（0-1，1表示严重茧房化）

请以JSON格式返回，格式如下：
{{
    \"summary\": \"总结\",
    \"topics\": [\"话题1\", \"话题2\"],
    \"bias_analysis\": \"偏见分析\",
    \"cocoon_score\": 0.8
}}",
            content_summary
        );

        let messages = vec![LLMMessage {
            role: "user".to_string(),
            content: prompt,
        }];

        let response = self.send_request(messages).await?;
        
        let analysis: ContentAnalysis = serde_json::from_str(&response)?;
        Ok(analysis)
    }

    pub async fn generate_reverse_search(&self, analysis: &ContentAnalysis) -> Result<ReverseSearch> {
        let prompt = format!(
            "基于以下内容分析，生成反向搜索词来打破信息茧房：

当前内容分析：
- 总结: {}
- 话题: {:?}
- 偏见分析: {}
- 茧房评分: {}

请生成：
1. 5-10个反向搜索词（与当前内容形成对比或补充）
2. 生成理由
3. 预期效果

请以JSON格式返回：
{{
    \"search_terms\": [\"搜索词1\", \"搜索词2\"],
    \"reasoning\": \"生成理由\",
    \"expected_benefits\": [\"效果1\", \"效果2\"]
}}",
            analysis.summary,
            analysis.topics,
            analysis.bias_analysis,
            analysis.cocoon_score
        );

        let messages = vec![LLMMessage {
            role: "user".to_string(),
            content: prompt,
        }];

        let response = self.send_request(messages).await?;
        
        let search: ReverseSearch = serde_json::from_str(&response)?;
        Ok(search)
    }

    pub async fn analyze_search_results(&self, videos: &[Video], search_term: &str) -> Result<String> {
        let content_summary = videos
            .iter()
            .map(|v| format!("标题: {}, 作者: {}", v.title, v.author))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "分析搜索词\"{}\"的结果，评估其对打破信息茧房的效果：

搜索结果：
{}

请提供：
1. 搜索结果与之前推荐内容的对比
2. 是否成功打破信息茧房
3. 建议的下一步行动（100字以内）",
            search_term, content_summary
        );

        let messages = vec![LLMMessage {
            role: "user".to_string(),
            content: prompt,
        }];

        self.send_request(messages).await
    }
}