// src/services/estimator.rs (version professionnelle)
use tiktoken_rs::get_bpe_from_model;
use std::sync::OnceLock;

static ENCODER: OnceLock<tiktoken_rs::CoreBPE> = OnceLock::new();

fn get_encoder() -> &'static tiktoken_rs::CoreBPE {
    ENCODER.get_or_init(|| {
        get_bpe_from_model("gpt-4").expect("Failed to load tiktoken encoder for gpt-4")
    })
}

pub fn estimate_tokens(prompt: &str) -> i32 {
    let encoder = get_encoder();
    let tokens = encoder.encode_with_special_tokens(prompt);
    tokens.len() as i32
}

pub fn estimate_cost(tokens: i32) -> f64 {
    // Prix GPT-4 (2024) : $0.03 par 1000 tokens input
    const PRICE_PER_1K: f64 = 0.03;
    (tokens as f64 / 1000.0) * PRICE_PER_1K
}

pub fn calculate_ai_cost_score(tokens: i32, cost: f64) -> i32 {
    // Score 0-100 : plus bas = meilleur coût
    // Référence : 0 tokens = 100, 1000 tokens = 70, 3000 tokens = 10
    let max_tokens = 4000; // max pour un prompt raisonnable
    let normalized = (tokens as f64 / max_tokens as f64).min(1.0);
    let score = ((1.0 - normalized) * 100.0) as i32;
    score.max(0).min(100)
}
