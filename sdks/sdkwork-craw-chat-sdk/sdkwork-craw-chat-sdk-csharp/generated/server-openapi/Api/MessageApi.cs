using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class MessageApi
    {
        private readonly SdkHttpClient _client;

        public MessageApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Edit a posted message
        /// </summary>
        public async Task<MessageMutationResult?> EditAsync(string messageId, EditMessageRequest body)
        {
            return await _client.PostAsync<MessageMutationResult>(ApiPaths.BackendPath($"/messages/{messageId}/edit"), body, null, null, "application/json");
        }

        /// <summary>
        /// Recall a posted message
        /// </summary>
        public async Task<MessageMutationResult?> RecallAsync(string messageId)
        {
            return await _client.PostAsync<MessageMutationResult>(ApiPaths.BackendPath($"/messages/{messageId}/recall"), null);
        }
    }
}
