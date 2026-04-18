using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class AbortStreamRequest
    {
        public int? FrameSeq { get; set; }
        public string? Reason { get; set; }
    }
}
