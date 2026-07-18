// src/services/estimator.rs
pub fn estimate_tokens(prompt: &str) -> i32 {
    // Approche simple : 1 token ≈ 4 caractères (pour MVP)
    // On peut améliorer avec tiktoken plus tard
    (prompt.len() as f64 / 4.0).ceil() as i32
}

pub fn estimate_cost(tokens: i32) -> f64 {
    // Prix simulé : 0.00002 $ par token (GPT-4 style)
    tokens as f64 * 0.00002
}

pub fn calculate_ai_cost_score(tokens: i32, cost: f64) -> i32 {
    // Score 0-100 : plus bas = meilleur
    // Exemple : 0 tokens = 100, 1000 tokens = 50, 2000 tokens = 0
    let score = 100 - ((tokens as f64 / 2000.0) * 100.0).min(100.0) as i32;
    score.max(0)
}
