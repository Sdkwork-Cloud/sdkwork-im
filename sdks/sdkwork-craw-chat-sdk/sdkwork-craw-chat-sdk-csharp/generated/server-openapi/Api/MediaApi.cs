using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class MediaApi
    {
        private readonly SdkHttpClient _client;

        public MediaApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Create a media upload record
        /// </summary>
        public async Task<MediaAsset?> CreateMediaUploadAsync(CreateUploadRequest body)
        {
            return await _client.PostAsync<MediaAsset>(ApiPaths.BackendPath("/media/uploads"), body, null, null, "application/json");
        }

        /// <summary>
        /// Complete a media upload
        /// </summary>
        public async Task<MediaAsset?> CompleteMediaUploadAsync(string mediaAssetId, CompleteUploadRequest body)
        {
            return await _client.PostAsync<MediaAsset>(ApiPaths.BackendPath($"/media/uploads/{mediaAssetId}/complete"), body, null, null, "application/json");
        }

        /// <summary>
        /// Issue a signed media download URL
        /// </summary>
        public async Task<MediaDownloadUrlResponse?> GetMediaDownloadUrlAsync(string mediaAssetId, Dictionary<string, object>? query = null)
        {
            return await _client.GetAsync<MediaDownloadUrlResponse>(ApiPaths.BackendPath($"/media/{mediaAssetId}/download-url"), query);
        }

        /// <summary>
        /// Get a media asset by id
        /// </summary>
        public async Task<MediaAsset?> GetMediaAssetAsync(string mediaAssetId)
        {
            return await _client.GetAsync<MediaAsset>(ApiPaths.BackendPath($"/media/{mediaAssetId}"));
        }

        /// <summary>
        /// Attach a ready media asset as a conversation message
        /// </summary>
        public async Task<PostMessageResult?> AttachMediaAssetAsync(string mediaAssetId, AttachMediaRequest body)
        {
            return await _client.PostAsync<PostMessageResult>(ApiPaths.BackendPath($"/media/{mediaAssetId}/attach"), body, null, null, "application/json");
        }
    }
}
