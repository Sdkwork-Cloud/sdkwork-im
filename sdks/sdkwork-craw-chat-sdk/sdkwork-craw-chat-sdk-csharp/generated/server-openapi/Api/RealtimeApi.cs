using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class RealtimeApi
    {
        private readonly SdkHttpClient _client;

        public RealtimeApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Replace realtime subscriptions for the current device
        /// </summary>
        public async Task<RealtimeSubscriptionSnapshot?> SyncRealtimeSubscriptionsAsync(SyncRealtimeSubscriptionsRequest body)
        {
            return await _client.PostAsync<RealtimeSubscriptionSnapshot>(ApiPaths.BackendPath("/realtime/subscriptions/sync"), body, null, null, "application/json");
        }

        /// <summary>
        /// Pull realtime events for the current device
        /// </summary>
        public async Task<RealtimeEventWindow?> ListRealtimeEventsAsync(Dictionary<string, object>? query = null)
        {
            return await _client.GetAsync<RealtimeEventWindow>(ApiPaths.BackendPath("/realtime/events"), query);
        }

        /// <summary>
        /// Ack realtime events for the current device
        /// </summary>
        public async Task<RealtimeAckState?> AckRealtimeEventsAsync(AckRealtimeEventsRequest body)
        {
            return await _client.PostAsync<RealtimeAckState>(ApiPaths.BackendPath("/realtime/events/ack"), body, null, null, "application/json");
        }
    }
}
