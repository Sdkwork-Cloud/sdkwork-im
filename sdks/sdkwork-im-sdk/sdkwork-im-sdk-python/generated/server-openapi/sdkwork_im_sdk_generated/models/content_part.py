from __future__ import annotations
from typing import TYPE_CHECKING, Optional, List, Dict, Any

if TYPE_CHECKING:
    from .data_content_part import DataContentPart
    from .media_content_part import MediaContentPart
    from .signal_content_part import SignalContentPart
    from .stream_ref_content_part import StreamRefContentPart
    from .text_content_part import TextContentPart


ContentPart = TextContentPart
