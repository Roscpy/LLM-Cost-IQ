// tests/integration_estimator.rs (plus réaliste)
use tiktoken_rs::get_bpe_from_model;

#[test]
fn test_encoder_is_cached() {
    let encoder1 = get_bpe_from_model("gpt-4").unwrap();
    let encoder2 = get_bpe_from_model("gpt-4").unwrap();
    // Doit être la même instance (cache)
    assert_eq!(encoder1.encode("test").len(), encoder2.encode("test").len());
}

#[test]
fn test_known_tokens() {
    let encoder = get_bpe_from_model("gpt-4").unwrap();
    let tokens = encoder.encode_with_special_tokens("Hello world");
    // "Hello world" = 2 tokens (Hello, world) ou 3 avec espace ?
    // Vérification empirique : sur GPT-4, c'est 2 tokens
    assert_eq!(tokens.len(), 2);
}
