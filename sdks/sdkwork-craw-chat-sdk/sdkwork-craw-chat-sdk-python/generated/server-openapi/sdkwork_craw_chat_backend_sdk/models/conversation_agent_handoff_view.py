from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class ConversationAgentHandoffView:
    status: str
    source: ConversationActorView
    target: ConversationActorView
    handoff_session_id: str
    handoff_reason: str = None
    accepted_at: str = None
    accepted_by: ConversationActorView = None
    resolved_at: str = None
    resolved_by: ConversationActorView = None
    closed_at: str = None
    closed_by: ConversationActorView = None
