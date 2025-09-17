// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

use asimov_module::{
    prelude::*,
    secrecy::{ExposeSecret, SecretString},
    tracing,
};
use core::error::Error;
use serde_json::{Value, json};

#[derive(Clone, Debug, bon::Builder)]
#[builder(on(String, into))]
pub struct Options {
    #[builder(default = "https://api.anthropic.com")]
    pub endpoint: String,

    #[builder(default = "claude-opus-4-1-20250805")]
    pub model: String,

    #[builder(default = 1024)]
    pub max_tokens: usize,

    #[builder(into)]
    pub api_key: SecretString,
}

pub fn chat(input: impl AsRef<str>, options: &Options) -> Result<Vec<String>, Box<dyn Error>> {
    let req = json!({
        "model": options.model,
        "max_tokens": options.max_tokens,
        "messages": [
            {"role": "user", "content": input.as_ref() }
        ]
    });

    let mut resp = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent("asimov-anthropic-module")
        .build()
        .new_agent()
        .post(options.endpoint.clone() + "/v1/messages")
        .header("x-api-key", options.api_key.expose_secret())
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .send_json(&req)
        .inspect_err(|e| tracing::error!("HTTP request failed: {e}"))?;
    tracing::debug!(response = ?resp);

    let status = resp.status();
    tracing::debug!(status = status.to_string());

    let resp: Value = resp
        .body_mut()
        .read_json()
        .inspect_err(|e| tracing::error!("unable to read HTTP response body: {e}"))?;
    tracing::debug!(body = ?resp);

    if !status.is_success() {
        tracing::error!("Received an error response: {status}");

        // {
        //   "type": "error",
        //   "error": {
        //     "type": "not_found_error",
        //     "message": "The requested resource could not be found."
        //   },
        //   "request_id": "req_..."
        // }
        if let Some(message) = resp["error"]["message"].as_str() {
            return Err(message.into());
        }
    }

    // {
    //   "id": "msg_...",
    //   "type": "message",
    //   "role": "assistant",
    //   "model": "claude-opus-4-1-20250805",
    //   "content": [
    //     {
    //       "type": "text",
    //       "text": "..."
    //     }
    //   ],
    //   "stop_reason": "max_tokens",
    //   "stop_sequence": null,
    //   "usage": {
    //     "input_tokens": 12,
    //     "cache_creation_input_tokens": 0,
    //     "cache_read_input_tokens": 0,
    //     "cache_creation": {
    //       "ephemeral_5m_input_tokens": 0,
    //       "ephemeral_1h_input_tokens": 0
    //     },
    //     "output_tokens": 32,
    //     "service_tier": "standard"
    //   }
    // }

    if let Some(stop_reason) = resp["stop_reason"].as_str() {
        tracing::debug!(stop_reason);
    }

    let responses = resp["content"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|c| c["type"].as_str().is_some_and(|t| t == "text"))
        .flat_map(|c| c["text"].as_str().map(ToString::to_string))
        .collect();

    Ok(responses)
}
