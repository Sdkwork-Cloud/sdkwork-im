package com.sdkwork.craw.chat.backend

data class SessionResumeView(
    val tenantId: String? = null,
    val actorId: String? = null,
    val actorKind: String? = null,
    val sessionId: String? = null,
    val deviceId: String? = null,
    val resumeRequired: Boolean? = null,
    val resumeFromSyncSeq: Int? = null,
    val latestSyncSeq: Int? = null,
    val resumedAt: String? = null,
    val presence: PresenceSnapshotView? = null
)
