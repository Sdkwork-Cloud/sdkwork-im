# Module Map

Use this page to move from a product requirement to the correct semantic SDK module and then down
to the exact App API page when needed.

## Task To Module Routing

| I need to... | Primary SDK module page | Related App API page | Typical client entrypoints |
| --- | --- | --- | --- |
| resume or disconnect a user session | [/sdk/modules/session-and-presence](/sdk/modules/session-and-presence) | [/api-reference/app/session-and-realtime](/api-reference/app/session-and-realtime) | `session.resume`, `session.disconnectDevice`, `session().resume` |
| update or read current presence | [/sdk/modules/session-and-presence](/sdk/modules/session-and-presence) | [/api-reference/app/session-and-realtime](/api-reference/app/session-and-realtime) | `presence.heartbeat`, `presence.current` |
| replace subscriptions or pull realtime events | [/sdk/modules/realtime](/sdk/modules/realtime) | [/api-reference/app/session-and-realtime](/api-reference/app/session-and-realtime) | `realtime.replaceSubscriptions`, `realtime.pullEvents`, `realtime().list_events` |
| register a device or read device sync | [/sdk/modules/devices-and-inbox](/sdk/modules/devices-and-inbox) | [/api-reference/app/device-sync](/api-reference/app/device-sync) | `devices.register`, `devices.getSyncFeed` |
| read the app inbox | [/sdk/modules/devices-and-inbox](/sdk/modules/devices-and-inbox) | [/api-reference/app/device-sync](/api-reference/app/device-sync) | `inbox.list`, `inbox().list` |
| create or inspect conversations | [/sdk/modules/conversations](/sdk/modules/conversations) | [/api-reference/app/conversations](/api-reference/app/conversations) | `conversations.create`, `conversations.get`, `conversations().create` |
| manage members or read cursors | [/sdk/modules/conversations](/sdk/modules/conversations) | [/api-reference/app/membership-and-read-state](/api-reference/app/membership-and-read-state) | `listMembers`, `addMember`, `updateReadCursor` |
| send, edit, or recall messages | [/sdk/modules/messages](/sdk/modules/messages) | [/api-reference/app/messages](/api-reference/app/messages) | `conversations.postText`, `messages.editText`, `messages.recall` |
| upload or attach media | [/sdk/modules/media](/sdk/modules/media) | [/api-reference/app/media](/api-reference/app/media) | `media.createUpload`, `media.completeUpload`, `media.attachText` |
| open or append stream frames | [/sdk/modules/streams](/sdk/modules/streams) | [/api-reference/app/streams](/api-reference/app/streams) | `streams.open`, `streams.appendTextFrame`, `streams.checkpoint` |
| create or coordinate RTC sessions | [/sdk/modules/rtc](/sdk/modules/rtc) | [/api-reference/app/rtc](/api-reference/app/rtc) | `rtc.create`, `rtc.postJsonSignal`, `rtc.issueParticipantCredential` |

## API Alignment

The SDK pages answer "how do I integrate this capability?".

The App API pages answer "what exact operations, request payloads, and response schemas back this
capability?".

Use both views together:

1. start with the SDK module page
2. use the matching App API page for exact payload and status details
3. return to the scenario pages when you need an end-to-end integration pattern

## Scenario Routing

- session and realtime bootstrap: [/sdk/examples/session-bootstrap](/sdk/examples/session-bootstrap)
- conversations and membership flow: [/sdk/examples/conversation-workflow](/sdk/examples/conversation-workflow)
- messages plus media: [/sdk/examples/message-and-media](/sdk/examples/message-and-media)
- streams plus RTC: [/sdk/examples/stream-and-rtc](/sdk/examples/stream-and-rtc)
