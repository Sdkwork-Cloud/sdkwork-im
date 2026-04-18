from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List, Dict, Any

@dataclass
class AgentHandoffStateView:
    tenant_id: str
    conversation_id: str
    status: str
    source: ChangeAgentHandoffStatusView
    target: ChangeAgentHandoffStatusView
    handoff_session_id: str
    handoff_reason: str = None
    accepted_at: str = None
    accepted_by: ChangeAgentHandoffStatusView = None
    resolved_at: str = None
    resolved_by: ChangeAgentHandoffStatusView = None
    closed_at: str = None
    closed_by: ChangeAgentHandoffStatusView = None
