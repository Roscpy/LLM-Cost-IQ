// src/services/estimator.rs (amélioration)
pub fn estimate_tokens(prompt: &str) -> i32 {
    // Version améliorée : comptage approximatif basé sur les mots + ponctuation
    // 1 token ≈ 1.3 mot (approche GPT)
    let word_count = prompt.split_whitespace().count();
    let char_count = prompt.chars().filter(|c| !c.is_whitespace()).count();
    // Moyenne pondérée : 70% mots, 30% caractères
    let tokens = (word_count as f64 * 1.3) + (char_count as f64 * 0.1);
    tokens.ceil() as i32
}
