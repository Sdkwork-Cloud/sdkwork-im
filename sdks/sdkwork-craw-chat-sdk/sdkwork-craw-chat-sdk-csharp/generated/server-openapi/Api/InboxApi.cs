using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class InboxApi
    {
        private readonly SdkHttpClient _client;

        public InboxApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Get inbox entries
        /// </summary>
        public async Task<InboxResponse?> GetInboxAsync()
        {
            return await _client.GetAsync<InboxResponse>(ApiPaths.BackendPath("/inbox"));
        }
    }
}
