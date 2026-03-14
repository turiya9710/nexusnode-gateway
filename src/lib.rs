use worker::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Define a minimal, unified incoming request structure
#[derive(Deserialize, Serialize, Debug)]
struct UnifiedRequest {
    model_tier: String, // e.g., "smart", "fast"
    messages: Vec<serde_json::Value>,
    stream: Option<bool>,
}

// Minimal structure for OpenAI-compatible success response
#[derive(Serialize)]
struct NexusResponse {
    provider: String,
    model_used: String,
    response_payload: serde_json::Value,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // 1. Setup panic hook for cleaner debug logs
    console_error_panic_hook::set_once();

    // 2. Validate the request method (MVP only allows POST)
    if req.method() != Method::Post {
        return Response::error("Method Not Allowed. Use POST.", 405);
    }

    // 3. Clone the request to read the body (which consumes it)
    let mut req_clone = req.clone()?;
    let unified_body: UnifiedRequest = match req_clone.json().await() {
        Ok(b) => b,
        Err(_) => return Response::error("Invalid JSON body", 400),
    };

    // 4. Retrieve Secrets from Environment
    let openai_key = match env.secret("OPENAI_API_KEY") {
        Ok(key) => key.to_string(),
        Err(_) => return Response::error("OPENAI_API_KEY secret not set", 500),
    };

    let anthropic_key = match env.secret("ANTHROPIC_API_KEY") {
        Ok(key) => key.to_string(),
        Err(_) => return Response::error("ANTHROPIC_API_KEY secret not set", 500),
    };

    // 5. Hardcoded Model Mapping (MVP Logic)
    // In later versions, this will come from Cloudflare D1
    let (primary_model, primary_url, primary_key) = ("gpt-4o", "https://api.openai.com/v1/chat/completions", openai_key);
    let (secondary_model, secondary_url, secondary_key) = ("claude-3-5-sonnet-20240620", "https://api.anthropic.com/v1/messages", anthropic_key);

    // ==========================================
    // --- ATTEMPT 1: PRIMARY (OPENAI) ---
    // ==========================================
    
    // Construct the OpenAI Payload
    let openai_payload = json!({
        "model": primary_model,
        "messages": unified_body.messages,
        "stream": false // MVP streaming is not supported
    });

    let mut headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", primary_key))?;
    headers.set("Content-Type", "application/json")?;

    let openai_request = Request::new_with_init(
        primary_url,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(openai_payload.to_string().into())),
    )?;

    // Make the first call
    let mut primary_response = Fetch::Url(openai_url.parse().unwrap()).send().await?;

    // If Primary Success, Return immediately
    if primary_response.status_code() == 200 {
        let json_body: serde_json::Value = primary_response.json().await()?;
        let nexus_resp = NexusResponse {
            provider: "openai".to_string(),
            model_used: primary_model.to_string(),
            response_payload: json_body,
        };
        return Response::from_json(&nexus_resp);
    }

    // ==========================================
    // --- ATTEMPT 2: FAILOVER (ANTHROPIC) ---
    // ==========================================
    
    // If Primary failed, we log it and proceed to secondary
    console_log!("Primary (OpenAI) failed (Status: {}). Attempting Failover.", primary_response.status_code());

    // Anthropic requires slightly different handling of 'system' vs 'messages'
    // This is a minimal conversion for the MVP
    let anthropic_payload = json!({
        "model": secondary_model,
        "max_tokens": 1024,
        "messages": unified_body.messages, // Assumes messages are already Anthropic-compatible
    });

    let mut a_headers = Headers::new();
    a_headers.set("x-api-key", &secondary_key)?;
    a_headers.set("anthropic-version", "2023-06-01")?;
    a_headers.set("Content-Type", "application/json")?;

    let anthropic_request = Request::new_with_init(
        secondary_url,
        RequestInit::new()
            .with_method(Method::Post)
            .with_headers(a_headers)
            .with_body(Some(anthropic_payload.to_string().into())),
    )?;

    let mut secondary_response = Fetch::Url(secondary_url.parse().unwrap()).send().await?;

    // If Secondary Success
    if secondary_response.status_code() == 200 {
        let json_body: serde_json::Value = secondary_response.json().await()?;
        let nexus_resp = NexusResponse {
            provider: "anthropic".to_string(),
            model_used: secondary_model.to_string(),
            response_payload: json_body,
        };
        return Response::from_json(&nexus_resp);
    }

    // ==========================================
    // --- BOTH FAILED ---
    // ==========================================
    Response::error("Both Primary and Secondary LLM providers failed.", 502)
}