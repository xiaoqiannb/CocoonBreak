use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub title: String,
    pub author: String,
    pub url: String,
    pub cover: String,
    pub duration: String,
    pub views: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    pub summary: String,
    pub topics: Vec<String>,
    pub bias_analysis: String,
    pub cocoon_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseSearch {
    pub search_terms: Vec<String>,
    pub reasoning: String,
    pub expected_benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchProgress {
    pub current_term: String,
    pub progress: f64,
    pub videos_found: Vec<Video>,
    pub status: String,
}