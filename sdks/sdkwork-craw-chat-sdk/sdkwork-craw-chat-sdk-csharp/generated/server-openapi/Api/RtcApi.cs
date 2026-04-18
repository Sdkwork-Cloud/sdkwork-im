using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Sdkwork.CrawChat.BackendSdk.Models;
using SdkHttpClient = Sdkwork.CrawChat.BackendSdk.Http.HttpClient;

namespace Sdkwork.CrawChat.BackendSdk.Api
{
    public class RtcApi
    {
        private readonly SdkHttpClient _client;

        public RtcApi(SdkHttpClient client)
        {
            _client = client;
        }

        /// <summary>
        /// Create an RTC session
        /// </summary>
        public async Task<RtcSession?> CreateRtcSessionAsync(CreateRtcSessionRequest body)
        {
            return await _client.PostAsync<RtcSession>(ApiPaths.BackendPath("/rtc/sessions"), body, null, null, "application/json");
        }

        /// <summary>
        /// Invite participants into an RTC session
        /// </summary>
        public async Task<RtcSession?> InviteRtcSessionAsync(string rtcSessionId, InviteRtcSessionRequest body)
        {
            return await _client.PostAsync<RtcSession>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/invite"), body, null, null, "application/json");
        }

        /// <summary>
        /// Accept an RTC session
        /// </summary>
        public async Task<RtcSession?> AcceptRtcSessionAsync(string rtcSessionId, UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<RtcSession>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/accept"), body, null, null, "application/json");
        }

        /// <summary>
        /// Reject an RTC session
        /// </summary>
        public async Task<RtcSession?> RejectRtcSessionAsync(string rtcSessionId, UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<RtcSession>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/reject"), body, null, null, "application/json");
        }

        /// <summary>
        /// End an RTC session
        /// </summary>
        public async Task<RtcSession?> EndRtcSessionAsync(string rtcSessionId, UpdateRtcSessionRequest body)
        {
            return await _client.PostAsync<RtcSession>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/end"), body, null, null, "application/json");
        }

        /// <summary>
        /// Post an RTC signaling event
        /// </summary>
        public async Task<RtcSignalEvent?> PostRtcSignalAsync(string rtcSessionId, PostRtcSignalRequest body)
        {
            return await _client.PostAsync<RtcSignalEvent>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/signals"), body, null, null, "application/json");
        }

        /// <summary>
        /// Issue an RTC participant credential
        /// </summary>
        public async Task<RtcParticipantCredential?> IssueRtcParticipantCredentialAsync(string rtcSessionId, IssueRtcParticipantCredentialRequest body)
        {
            return await _client.PostAsync<RtcParticipantCredential>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/credentials"), body, null, null, "application/json");
        }

        /// <summary>
        /// Get the RTC recording artifact
        /// </summary>
        public async Task<RtcRecordingArtifact?> GetRtcRecordingArtifactAsync(string rtcSessionId)
        {
            return await _client.GetAsync<RtcRecordingArtifact>(ApiPaths.BackendPath($"/rtc/sessions/{rtcSessionId}/artifacts/recording"));
        }
    }
}
