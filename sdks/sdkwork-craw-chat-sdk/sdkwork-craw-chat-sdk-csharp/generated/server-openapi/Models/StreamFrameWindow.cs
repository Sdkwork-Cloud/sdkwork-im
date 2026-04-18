using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace Sdkwork.CrawChat.BackendSdk.Models
{
    public class StreamFrameWindow
    {
        public List<StreamFrame>? Items { get; set; }
        public int? NextAfterFrameSeq { get; set; }
        public bool? HasMore { get; set; }
    }
}
