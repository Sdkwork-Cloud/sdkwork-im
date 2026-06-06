import { GoogleGenAI } from '@google/genai';
import type { IncomingMessage, ServerResponse } from 'http';

const LOCAL_APP_MODULES = [
  'chat',
  'workspace',
  'orders',
  'shop',
  'calendar',
  'notary',
  'knowledge',
  'enterprise',
  'devices',
  'community',
  'voice',
  'agent',
  'course',
  'contacts',
  'favorites',
] as const;

function createGoogleAiClient() {
  return new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
}

let googleAiClient: ReturnType<typeof createGoogleAiClient> | null = null;

function getGoogleAiClient(): ReturnType<typeof createGoogleAiClient> {
  if (!process.env.GEMINI_API_KEY) {
    throw new Error('GEMINI_API_KEY is not configured.');
  }
  googleAiClient ??= createGoogleAiClient();
  return googleAiClient;
}

function normalizeString(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function createDocumentPrompt(payload: Record<string, unknown>): string | null {
  const action = normalizeString(payload.action);
  const content = normalizeString(payload.content);
  const context = normalizeString(payload.context) || 'None';
  const instruction = normalizeString(payload.instruction);

  if (action === 'rewrite') {
    return `You are an expert document editor agent. Rewrite the following text to make it more professional, clear, and well-structured. Return ONLY the rewritten text without markdown fences, or extra commentary.\n\nText: ${content}`;
  }
  if (action === 'summarize') {
    return `You are an expert document editor agent. Summarize the following text into key bullet points. Return ONLY the summarized points without markdown fences, or extra commentary.\n\nText: ${content}`;
  }
  if (action === 'expand') {
    return `You are an expert document editor agent. Expand the following text, providing more details, examples, and context. Return ONLY the expanded text without markdown fences, or extra commentary.\n\nText: ${content}`;
  }
  if (action === 'translate') {
    return `You are an expert document editor agent. Translate the following text into fluent English if it is Chinese, or Chinese if it is English. Return ONLY the translated text without markdown fences, or extra commentary.\n\nText: ${content}`;
  }
  if (action === 'instruct') {
    return [
      'You are an expert document editor agent. You are currently editing a document.',
      `Context, surrounding text, or previous text: ${context}`,
      `Instruction from user: ${instruction}`,
      `Current selected text, if any: ${content || 'None'}`,
      'Follow the user instruction and generate the necessary Markdown text.',
      'Return ONLY the modified or generated text without conversational filler or enclosing markdown fences.',
    ].join('\n\n');
  }
  return null;
}

function sendJson(res: ServerResponse, statusCode: number, body: unknown): void {
  res.statusCode = statusCode;
  res.setHeader('Content-Type', 'application/json; charset=utf-8');
  res.end(JSON.stringify(body));
}

function readJsonBody(req: IncomingMessage): Promise<Record<string, unknown>> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on('data', (chunk) => chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk)));
    req.on('error', reject);
    req.on('end', () => {
      const rawBody = Buffer.concat(chunks).toString('utf8').trim();
      if (!rawBody) {
        resolve({});
        return;
      }
      try {
        const parsed = JSON.parse(rawBody);
        resolve(parsed && typeof parsed === 'object' && !Array.isArray(parsed) ? parsed : {});
      } catch (error) {
        reject(error);
      }
    });
  });
}

async function handleDocumentAgent(req: IncomingMessage, res: ServerResponse): Promise<void> {
  const prompt = createDocumentPrompt(await readJsonBody(req));
  if (!prompt) {
    sendJson(res, 400, { error: 'Invalid action' });
    return;
  }
  const response = await getGoogleAiClient().models.generateContent({
    model: 'gemini-2.5-flash',
    contents: prompt,
  });
  sendJson(res, 200, { result: response.text });
}

async function handleIconAgent(req: IncomingMessage, res: ServerResponse): Promise<void> {
  const payload = await readJsonBody(req);
  const description = normalizeString(payload.description);
  const prompt = `A highly stylized, minimalist icon for a knowledge base about: ${description}. White or clear background, app icon style, modern, sleek, simple.`;
  const response = await getGoogleAiClient().models.generateContent({
    model: 'gemini-2.5-flash-image',
    contents: prompt,
  });

  let imageUrl: string | null = null;
  for (const part of response?.candidates?.[0]?.content?.parts ?? []) {
    if (part.inlineData) {
      imageUrl = `data:${part.inlineData.mimeType};base64,${part.inlineData.data}`;
      break;
    }
  }
  sendJson(res, imageUrl ? 200 : 500, imageUrl ? { result: imageUrl } : { error: 'No image generated' });
}

export async function handleSdkworkChatLocalApiRequest(
  req: IncomingMessage,
  res: ServerResponse,
  requestPath = new URL(req.url ?? '/', 'http://127.0.0.1').pathname,
): Promise<boolean> {
  try {
    if (requestPath === '/api/config/modules' && req.method === 'GET') {
      sendJson(res, 200, { modules: [...LOCAL_APP_MODULES] });
      return true;
    }
    if (requestPath === '/api/agent/doc' && req.method === 'POST') {
      await handleDocumentAgent(req, res);
      return true;
    }
    if (requestPath === '/api/agent/icon' && req.method === 'POST') {
      await handleIconAgent(req, res);
      return true;
    }
    return false;
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Local API processing failed';
    sendJson(res, message.includes('GEMINI_API_KEY') ? 503 : 500, { error: message });
    return true;
  }
}
