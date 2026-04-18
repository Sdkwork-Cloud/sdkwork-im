import { CrawChatSdkClient } from '../dist/index.js';

const sdk = new CrawChatSdkClient({
  baseUrl: 'https://api.example.test',
  authToken: 'token-1',
});

async function verifyLiveContract() {
  const live = await sdk.connect({
    deviceId: 'device-1',
    subscriptions: {
      conversations: ['conversation-1'],
      rtcSessions: ['rtc-1'],
    },
  });

  live.messages.on((message, context) => {
    const messageType: string = message.type;
    const sequence: number = context.sequence;
    void messageType;
    void sequence;
  });

  live.messages.onConversation('conversation-1', (message, context) => {
    const messageType: string = message.type;
    const conversationId: string | undefined = context.conversationId;
    void messageType;
    void conversationId;
  });

  live.data.on((data, context) => {
    const schemaRef: string | undefined = data.schemaRef;
    const sequence: number = context.sequence;
    void schemaRef;
    void sequence;
  });

  live.signals.on((signal, context) => {
    const signalType: string = signal.signalType;
    const scopeId: string = context.scopeId;
    void signalType;
    void scopeId;
  });

  live.signals.onRtcSession('rtc-1', (signal, context) => {
    const signalType: string = signal.signalType;
    const scopeId: string = context.scopeId;
    void signalType;
    void scopeId;
  });

  live.events.on((context) => {
    const kind: 'message' | 'data' | 'signal' | 'unknown' = context.kind;
    void kind;
  });

  live.lifecycle.onStateChange((state) => {
    const status: 'connected' | 'error' | 'closed' = state.status;
    void status;
  });

  live.lifecycle.onError((context) => {
    const code: string = context.code;
    void code;
  });

  const status: 'connected' | 'error' | 'closed' = live.lifecycle.getState().status;
  void status;

  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onMessage(() => {});
  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onConversationMessage('conversation-1', () => {});
  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onData(() => {});
  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onSignal(() => {});
  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onRawEvent(() => {});
  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onStateChange(() => {});
  // @ts-expect-error legacy flat callbacks were removed from the public live contract.
  live.onError(() => {});
  // @ts-expect-error resolved live handles do not expose a connecting lifecycle state.
  const connecting: 'connecting' = live.lifecycle.getState().status;
  void connecting;
}

void verifyLiveContract();
