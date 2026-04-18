package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class InboxApi(private val client: HttpClient) {

    /** Get inbox entries */
    suspend fun getInbox(): InboxResponse? {
        val raw = client.get(ApiPaths.backendPath("/inbox"))
        return client.convertValue(raw, object : TypeReference<InboxResponse>() {})
    }
}
