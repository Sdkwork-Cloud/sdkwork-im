using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class UpdateReadCursorRequest
    {
        public int? ReadSeq { get; set; }
        public string? LastReadMessageId { get; set; }
    }
}
