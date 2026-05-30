using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.AppApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.AppApi.Generated.Api
{
    public class IotApi
    {
        private readonly SdkHttpClient _client;

        public IotApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Retrieve IoT access provider health
        /// </summary>
        public async Task<Dictionary<string, object>?> AccessProviderHealthRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/iot/access/provider_health"));
        }

        /// <summary>
        /// Retrieve IoT protocol provider health
        /// </summary>
        public async Task<Dictionary<string, object>?> ProtocolProviderHealthRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/iot/protocol/provider_health"));
        }

        /// <summary>
        /// Ingest IoT protocol uplink
        /// </summary>
        public async Task<Dictionary<string, object>?> ProtocolUplinkCreateAsync()
        {
            return await _client.PostAsync<Dictionary<string, object>>(ApiPaths.AppPath("/iot/protocol/uplink"), null);
        }

        /// <summary>
        /// Ingest IoT protocol downlink
        /// </summary>
        public async Task<Dictionary<string, object>?> ProtocolDownlinkCreateAsync()
        {
            return await _client.PostAsync<Dictionary<string, object>>(ApiPaths.AppPath("/iot/protocol/downlink"), null);
        }



    }
}
