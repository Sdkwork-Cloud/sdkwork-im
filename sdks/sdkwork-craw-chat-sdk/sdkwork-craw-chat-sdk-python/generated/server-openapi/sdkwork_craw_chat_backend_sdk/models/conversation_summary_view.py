from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ConversationSummaryView:
    tenant_id: str
    conversation_id: str
    message_count: int
    last_message_seq: int
    last_message_id: str = None
    last_sender_id: str = None
    last_sender_kind: str = None
    last_sender: SummarySenderView = None
    last_summary: str = None
    last_message_at: str = None
    agent_handoff: ConversationAgentHandoffView = None
