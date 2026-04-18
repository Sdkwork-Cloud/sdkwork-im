use serde_json::Value;
use sdkwork_craw_chat_backend_sdk::{
  AppendStreamFrameRequest,
  ContentPart,
  EditMessageRequest,
  PostMessageRequest,
  PostRtcSignalRequest,
};

pub const DEFAULT_TEXT_FRAME_ENCODING: &str = "text/plain; charset=utf-8";

#[derive(Debug, Clone, Default)]
pub struct PostTextOptions {
  pub client_msg_id: Option<String>,
  pub summary: Option<String>,
  pub parts: Option<Vec<ContentPart>>,
  pub render_hints: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default)]
pub struct EditTextOptions {
  pub summary: Option<String>,
  pub parts: Option<Vec<ContentPart>>,
  pub render_hints: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default)]
pub struct TextFrameOptions {
  pub schema_ref: Option<String>,
  pub encoding: Option<String>,
  pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default)]
pub struct JsonRtcSignalOptions {
  pub schema_ref: Option<String>,
  pub signaling_stream_id: Option<String>,
  pub pretty: bool,
}

pub fn build_text_message(text: impl Into<String>, options: PostTextOptions) -> PostMessageRequest {
  PostMessageRequest {
    client_msg_id: options.client_msg_id,
    summary: options.summary,
    text: Some(text.into()),
    parts: options.parts,
    render_hints: options.render_hints,
  }
}

pub fn build_text_edit(text: impl Into<String>, options: EditTextOptions) -> EditMessageRequest {
  EditMessageRequest {
    summary: options.summary,
    text: Some(text.into()),
    parts: options.parts,
    render_hints: options.render_hints,
  }
}

pub fn build_text_stream_frame(
  frame_seq: i64,
  text: impl Into<String>,
  options: TextFrameOptions,
) -> AppendStreamFrameRequest {
  AppendStreamFrameRequest {
    frame_seq,
    frame_type: "text".to_string(),
    schema_ref: options.schema_ref,
    encoding: options
      .encoding
      .unwrap_or_else(|| DEFAULT_TEXT_FRAME_ENCODING.to_string()),
    payload: text.into(),
    attributes: options.attributes,
  }
}

pub fn build_json_rtc_signal(
  signal_type: impl Into<String>,
  payload: &Value,
  options: JsonRtcSignalOptions,
) -> PostRtcSignalRequest {
  let payload = if options.pretty {
    serde_json::to_string_pretty(payload).expect("serialize rtc signal payload")
  } else {
    serde_json::to_string(payload).expect("serialize rtc signal payload")
  };

  PostRtcSignalRequest {
    signal_type: signal_type.into(),
    schema_ref: options.schema_ref,
    payload,
    signaling_stream_id: options.signaling_stream_id,
  }
}
