// tests/estimator_test.rs
#[tokio::test]
async fn test_tiktoken_accuracy() {
    let prompt = "Hello world, this is a test prompt for LLM cost estimation.";
    let tokens = estimator::estimate_tokens(prompt);
    // Sur GPT-4, ce prompt devrait faire environ 12-14 tokens
    assert!(tokens >= 10 && tokens <= 20, "Tokens = {}", tokens);
    
    let cost = estimator::estimate_cost(tokens);
    assert!(cost > 0.0 && cost < 0.001); // < $0.001 pour ce petit prompt
}
