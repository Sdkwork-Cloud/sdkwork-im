using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class CompleteStreamRequest
    {
        public int? FrameSeq { get; set; }
        public string? ResultMessageId { get; set; }
    }
}
