using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.Im.AppApi.Generated.Models;
using SdkHttpClient = Sdkwork.Im.AppApi.Generated.Http.HttpClient;

namespace Sdkwork.Im.AppApi.Generated.Api
{
    public class RtcApi
    {
        private readonly SdkHttpClient _client;

        public RtcApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Map RTC provider callback
        /// </summary>
        public async Task<Dictionary<string, object>?> ProviderCallbacksCreateAsync()
        {
            return await _client.PostAsync<Dictionary<string, object>>(ApiPaths.AppPath("/rtc/provider_callbacks"), null);
        }

        /// <summary>
        /// Retrieve RTC provider health
        /// </summary>
        public async Task<Dictionary<string, object>?> ProviderHealthRetrieveAsync()
        {
            return await _client.GetAsync<Dictionary<string, object>>(ApiPaths.AppPath("/rtc/provider_health"));
        }



    }
}
