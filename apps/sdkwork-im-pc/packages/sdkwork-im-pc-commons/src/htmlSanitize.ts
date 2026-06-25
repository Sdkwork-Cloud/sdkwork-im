const BLOCKED_TAGS = new Set([
  'script',
  'iframe',
  'object',
  'embed',
  'link',
  'meta',
  'base',
  'form',
  'style',
  'svg',
  'math',
  'foreignobject',
]);

const BLOCKED_URI_PREFIXES = ['javascript:', 'data:', 'vbscript:'];

function isBlockedUri(value: string): boolean {
  const normalized = value.trim().toLowerCase();
  return BLOCKED_URI_PREFIXES.some((prefix) => normalized.startsWith(prefix));
}

function sanitizeElementTree(root: ParentNode): void {
  for (const blockedTag of BLOCKED_TAGS) {
    root.querySelectorAll(blockedTag).forEach((node) => node.remove());
  }

  root.querySelectorAll('*').forEach((element) => {
    for (const attribute of [...element.attributes]) {
      const attributeName = attribute.name.toLowerCase();
      if (attributeName.startsWith('on') || attributeName === 'srcdoc') {
        element.removeAttribute(attribute.name);
        continue;
      }
      if (attributeName === 'href' || attributeName === 'src' || attributeName === 'xlink:href') {
        if (isBlockedUri(attribute.value)) {
          element.removeAttribute(attribute.name);
        }
      }
    }
  });
}

export function sanitizeHtmlForDisplay(html: string): string {
  const trimmed = html.trim();
  if (!trimmed) {
    return '';
  }

  if (typeof DOMParser !== 'undefined') {
    const document = new DOMParser().parseFromString(trimmed, 'text/html');
    sanitizeElementTree(document);
    return document.body.innerHTML;
  }

  return trimmed
    .replace(/<script[\s\S]*?>[\s\S]*?<\/script>/giu, '')
    .replace(/<iframe[\s\S]*?>[\s\S]*?<\/iframe>/giu, '')
    .replace(/<svg[\s\S]*?>[\s\S]*?<\/svg>/giu, '')
    .replace(/<math[\s\S]*?>[\s\S]*?<\/math>/giu, '')
    .replace(/<style[\s\S]*?>[\s\S]*?<\/style>/giu, '')
    .replace(/\son[a-z]+\s*=\s*(['"])[\s\S]*?\1/giu, '')
    .replace(/\s(href|src)\s*=\s*(['"])\s*javascript:[\s\S]*?\2/giu, '')
    .replace(/\s(href|src)\s*=\s*(['"])\s*data:[\s\S]*?\2/giu, '')
    .replace(/javascript:/giu, '');
}
