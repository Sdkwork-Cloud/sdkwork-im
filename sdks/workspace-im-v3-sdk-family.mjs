import { spawnSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml, resolveGeneratorRoot } from './workspace-sdk-generator-root-shared.mjs';
import { loadOpenApiDocument } from './workspace-openapi-source-shared.mjs';

export const officialLanguages = [
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
];

const manifestByLanguage = {
  typescript: 'package.json',
  flutter: 'pubspec.yaml',
  rust: 'Cargo.toml',
  java: 'pom.xml',
  csharp: null,
  swift: 'Package.swift',
  kotlin: 'build.gradle.kts',
  go: 'go.mod',
  python: 'pyproject.toml',
};

const languageDisplayName = {
  typescript: 'TypeScript',
  flutter: 'Flutter',
  rust: 'Rust',
  java: 'Java',
  csharp: 'C#',
  swift: 'Swift',
  kotlin: 'Kotlin',
  go: 'Go',
  python: 'Python',
};

const textExtensions = new Set([
  '.bat',
  '.cs',
  '.csproj',
  '.dart',
  '.gradle',
  '.go',
  '.java',
  '.json',
  '.kt',
  '.kts',
  '.md',
  '.mjs',
  '.ps1',
  '.py',
  '.rs',
  '.sh',
  '.swift',
  '.toml',
  '.ts',
  '.txt',
  '.xml',
  '.yaml',
  '.yml',
]);

const forbiddenGeneratedAuthSurfaceText = [
  'setApiKey',
  'set_api_key',
  'SetApiKey',
  'apiKey?:',
  'apiKeyHeader',
  'apiKeyAsBearer',
  'api_key ->',
  'ApiKey      string',
  'API_KEY_HEADER',
  'API_KEY_USE_BEARER',
  'DEFAULT_API_KEY_HEADER',
  'DEFAULT_API_KEY_USE_BEARER',
  'defaultApiKeyHeader',
  'defaultApiKeyUseBearer',
  'ApiKeyHeader',
  'ApiKeyUseBearer',
  'Authentication Modes',
  'Professional TypeScript SDK for SDKWork API',
  'Professional Flutter SDK for SDKWork API',
  'Professional Rust SDK for SDKWork API',
  'Professional Java SDK for SDKWork API',
  'Professional C# SDK for SDKWork API',
  'Professional Swift SDK for SDKWork API',
  'Professional Kotlin SDK for SDKWork API',
  'Professional Go SDK for SDKWork API',
  'Professional Python SDK for SDKWork API',
  'your-api-key',
  'Mode A: API Key',
];

function fail(prefix, message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

function run(prefix, command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    encoding: options.capture ? 'utf8' : undefined,
    stdio: options.capture ? ['ignore', 'pipe', 'pipe'] : 'inherit',
    shell: false,
  });
  if (result.error) {
    fail(prefix, `${options.step || command} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    if (options.capture && result.stderr) {
      process.stderr.write(result.stderr);
    }
    fail(prefix, `${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(prefix, `${options.step || command} terminated with signal ${result.signal}`);
  }
  return options.capture ? String(result.stdout || '').trim() : '';
}

function parseLanguages(prefix, values) {
  const normalized = [];
  for (const value of values) {
    for (const segment of String(value || '').split(',')) {
      const language = segment.trim().toLowerCase();
      if (language) {
        normalized.push(language);
      }
    }
  }
  const languages = normalized.length > 0 ? normalized : officialLanguages;
  for (const language of languages) {
    if (!officialLanguages.includes(language)) {
      fail(prefix, `Unsupported language: ${language}`);
    }
  }
  return [...new Set(languages)];
}

function parseGenerateArgs(prefix, argv) {
  const parsed = {
    languages: [],
    fixedSdkVersion: '',
    baseUrl: 'http://127.0.0.1:18090',
    schemaUrl: '',
    refreshLive: false,
  };
  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      parsed.languages.push(argv[index + 1] || '');
      index += 1;
      continue;
    }
    if (current === '--fixed-sdk-version') {
      parsed.fixedSdkVersion = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--base-url') {
      parsed.baseUrl = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--schema-url') {
      parsed.schemaUrl = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--refresh-live') {
      parsed.refreshLive = true;
      continue;
    }
    fail(prefix, `Unknown argument: ${current}`);
  }
  return parsed;
}

function parseVerifyArgs(prefix, argv) {
  const parsed = { languages: [] };
  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      parsed.languages.push(argv[index + 1] || '');
      index += 1;
      continue;
    }
    fail(prefix, `Unknown argument: ${current}`);
  }
  return parsed;
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8').replace(/^\uFEFF/, '');
}

function readJson(filePath) {
  return JSON.parse(readText(filePath));
}

function escapeRegExp(source) {
  return source.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function writeTextIfChanged(filePath, source) {
  const normalized = source.endsWith('\n') ? source : `${source}\n`;
  if (existsSync(filePath) && readText(filePath) === normalized) {
    return;
  }
  writeFileSync(filePath, normalized, 'utf8');
}

function writeJsonIfChanged(filePath, value) {
  writeTextIfChanged(filePath, JSON.stringify(value, null, 2));
}

function relativePath(root, filePath) {
  return path.relative(root, filePath).replace(/\\/g, '/');
}

function generatedDescriptionFor(config, language) {
  if (!config.generatedApiLabel) {
    return null;
  }
  return `Generator-owned ${languageDisplayName[language] || language} transport SDK for the ${config.generatedApiLabel}.`;
}

function normalizeAssemblyDescriptions(root, config) {
  if (!config.generatedApiLabel) {
    return;
  }
  const assemblyPath = path.join(root, '.sdkwork-assembly.json');
  if (!existsSync(assemblyPath)) {
    return;
  }
  const assembly = readJson(assemblyPath);
  let changed = false;
  for (const entry of assembly.languages ?? []) {
    const expected = generatedDescriptionFor(config, entry.language);
    if (expected && entry.description !== expected) {
      entry.description = expected;
      changed = true;
    }
  }
  if (changed) {
    writeFileSync(assemblyPath, `${JSON.stringify(assembly, null, 2)}\n`, 'utf8');
  }
}

function renderGeneratedTypeScriptReadme(config) {
  const description = generatedDescriptionFor(config, 'typescript')
    || `Generator-owned TypeScript transport SDK for ${config.sdkName}.`;
  return `# ${config.sdkName}

${description}

This package is generated transport. It targets \`${config.apiPrefix}\` and is not a login,
user, tenant, organization, or account-session SDK. Those identity and token lifecycles are
owned by \`sdkwork-appbase\`; this SDK only forwards the already validated dual-token context.

## Install

\`\`\`bash
npm install ${config.packages.typescript}
\`\`\`

## Usage

\`\`\`typescript
import { ${config.primaryClient} } from '${config.packages.typescript}';

const client = new ${config.primaryClient}({
  baseUrl: 'http://127.0.0.1:18090',
  authToken: appbaseAuthToken,
  accessToken: appbaseAccessToken,
});

client.setAuthToken(appbaseAuthToken);
client.setAccessToken(appbaseAccessToken);
\`\`\`

## Token Boundary

- \`Authorization: Bearer <authToken>\` carries the upstream authenticated principal context.
- \`Access-Token: <accessToken>\` carries the upstream access token context.
- Login, refresh, current-user, tenant, organization, and account-session APIs stay outside this package.

## Regeneration Contract

- Generated files are tracked by the SDK generator under \`.sdkwork/\`.
- Fix runtime, OpenAPI, or family generator inputs first, then regenerate.
- Hand-written application wrappers must live outside \`generated/server-openapi\`.
`;
}

function renderGeneratedReadme(config, language) {
  const description = generatedDescriptionFor(config, language)
    || `Generator-owned ${languageDisplayName[language] || language} transport SDK for ${config.sdkName}.`;
  return `# ${config.sdkName}

${description}

This package is generated transport. It targets \`${config.apiPrefix}\` and is not a login,
user, tenant, organization, or account-session SDK. Those identity and token lifecycles are
owned by \`sdkwork-appbase\`; this SDK only forwards the already validated dual-token context.

## Token Boundary

- \`Authorization: Bearer <authToken>\` carries the upstream authenticated principal context.
- \`Access-Token: <accessToken>\` carries the upstream access token context.
- Login, refresh, current-user, tenant, organization, and account-session APIs stay outside this package.

## Regeneration Contract

- Generated files are tracked by the SDK generator under \`.sdkwork/\`.
- Fix runtime, OpenAPI, or family generator inputs first, then regenerate.
- Hand-written application wrappers must live outside \`generated/server-openapi\`.
`;
}

function replaceBetween(source, startPattern, endPattern, replacement, label) {
  const startMatch = startPattern instanceof RegExp ? startPattern.exec(source) : null;
  const startIndex = startMatch ? startMatch.index : source.indexOf(startPattern);
  const startLength = startMatch ? startMatch[0].length : String(startPattern).length;
  if (startIndex < 0) {
    return source;
  }
  const afterStart = startIndex + startLength;
  const endMatch = endPattern instanceof RegExp
    ? endPattern.exec(source.slice(afterStart))
    : null;
  const relativeEndIndex = endMatch ? endMatch.index : source.indexOf(endPattern, afterStart) - afterStart;
  if (relativeEndIndex < 0) {
    fail('sdkwork-im-v3-sdk-family', `Unable to normalize ${label}; end marker not found.`);
  }
  const endIndex = afterStart + relativeEndIndex;
  return `${source.slice(0, startIndex)}${replacement}${source.slice(endIndex)}`;
}

function normalizeGenericReadme(root, config, language) {
  const outputDir = languageOutputDir(root, config, language);
  const readmePath = path.join(outputDir, 'README.md');
  writeTextIfChanged(readmePath, renderGeneratedReadme(config, language));
}

function updateTextFileIfExists(filePath, updater) {
  if (!existsSync(filePath)) {
    return;
  }
  writeTextIfChanged(filePath, updater(readText(filePath)));
}

function normalizeGeneratedManifestDescription(root, config, language) {
  const expectedDescription = generatedDescriptionFor(config, language);
  if (!expectedDescription) {
    return;
  }

  const outputDir = languageOutputDir(root, config, language);
  const manifestName = manifestNameFor(config, language);
  if (!manifestName) {
    return;
  }

  const manifestPath = path.join(outputDir, manifestName);
  if (!existsSync(manifestPath)) {
    return;
  }

  if (language === 'typescript') {
    const manifest = readJson(manifestPath);
    let changed = false;
    if (manifest.description !== expectedDescription) {
      manifest.description = expectedDescription;
      changed = true;
    }
    if (manifest.private !== true) {
      manifest.private = true;
      changed = true;
    }
    if (changed) {
      writeJsonIfChanged(manifestPath, manifest);
    }
    return;
  }

  if (language === 'flutter') {
    updateTextFileIfExists(manifestPath, (source) => {
      if (/^description:\s*.*$/m.test(source)) {
        return source.replace(/^description:\s*.*$/m, `description: ${expectedDescription}`);
      }
      return source.replace(/^(version:\s*.*)$/m, `$1\ndescription: ${expectedDescription}`);
    });
    return;
  }

  if (language === 'rust') {
    updateTextFileIfExists(manifestPath, (source) => {
      if (/^description\s*=\s*"[^"]*"\s*$/m.test(source)) {
        return source.replace(/^description\s*=\s*"[^"]*"\s*$/m, `description = "${expectedDescription}"`);
      }
      return source.replace(/^(version\s*=\s*"[^"]*"\s*)$/m, `$1\ndescription = "${expectedDescription}"`);
    });
    return;
  }

  if (language === 'java') {
    updateTextFileIfExists(manifestPath, (source) => source.replace(
      /<description>[^<]*<\/description>/,
      `<description>${expectedDescription}</description>`,
    ));
    return;
  }

  if (language === 'csharp') {
    updateTextFileIfExists(manifestPath, (source) => source.replace(
      /<Description>[^<]*<\/Description>/,
      `<Description>${expectedDescription}</Description>`,
    ));
    return;
  }

  if (language === 'kotlin') {
    updateTextFileIfExists(manifestPath, (source) => {
      if (/^description\s*=\s*"[^"]*"\s*$/m.test(source)) {
        return source.replace(/^description\s*=\s*"[^"]*"\s*$/m, `description = "${expectedDescription}"`);
      }
      return source.replace(/^(version\s*=\s*"[^"]*"\s*)$/m, `$1\ndescription = "${expectedDescription}"`);
    });
    return;
  }

  if (language === 'python') {
    updateTextFileIfExists(manifestPath, (source) => {
      if (/^description\s*=\s*"[^"]*"\s*$/m.test(source)) {
        return source.replace(/^description\s*=\s*"[^"]*"\s*$/m, `description = "${expectedDescription}"`);
      }
      return source.replace(/^(version\s*=\s*"[^"]*"\s*)$/m, `$1\ndescription = "${expectedDescription}"`);
    });
  }
}

function normalizeGeneratedSwiftPackageManifest(root, config) {
  const packageName = languagePackageName(config, 'swift');
  if (!packageName) {
    return;
  }

  const manifestPath = path.join(languageOutputDir(root, config, 'swift'), 'Package.swift');
  updateTextFileIfExists(manifestPath, (source) => source
    .replace(/name:\s*"ImSDK"/g, `name: "${packageName}"`)
    .replace(/targets:\s*\["ImSDK"\]/g, `targets: ["${packageName}"]`)
    .replace(/name:\s*"ImSdkGenerated"/g, `name: "${packageName}"`)
    .replace(/targets:\s*\["ImSdkGenerated"\]/g, `targets: ["${packageName}"]`));
}

function normalizeGeneratedSdkMetadata(root, config, language) {
  const expectedDescription = generatedDescriptionFor(config, language);
  if (!expectedDescription) {
    return;
  }

  const sdkMetadataPath = path.join(languageOutputDir(root, config, language), 'sdkwork-sdk.json');
  if (!existsSync(sdkMetadataPath)) {
    return;
  }

  const metadata = readJson(sdkMetadataPath);
  if (metadata.description !== expectedDescription) {
    metadata.description = expectedDescription;
    writeJsonIfChanged(sdkMetadataPath, metadata);
  }
}

function normalizeGeneratedPackageMetadata(root, config, language) {
  normalizeGeneratedManifestDescription(root, config, language);
  if (language === 'swift') {
    normalizeGeneratedSwiftPackageManifest(root, config);
  }
  normalizeGeneratedSdkMetadata(root, config, language);
}

function normalizeGeneratedTypeScriptAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'typescript');
  if (!existsSync(outputDir)) {
    return;
  }

  const sdkSourcePath = path.join(outputDir, 'src', 'sdk.ts');
  if (existsSync(sdkSourcePath)) {
    const source = readText(sdkSourcePath).replace(
      /\n  setApiKey\(apiKey: string\): this \{\r?\n    this\.httpClient\.setApiKey\(apiKey\);\r?\n    return this;\r?\n  \}\r?\n/g,
      '\n',
    );
    writeTextIfChanged(sdkSourcePath, source);
  }

  const httpClientSourcePath = path.join(outputDir, 'src', 'http', 'client.ts');
  if (existsSync(httpClientSourcePath)) {
    let source = readText(httpClientSourcePath)
      .replace(/\n  private static readonly API_KEY_HEADER: string = 'Access-Token';\r?\n/g, '\n')
      .replace(/\n  private static readonly API_KEY_USE_BEARER = false;\r?\n/g, '\n')
      .replace(
        /\n  setApiKey\(apiKey: string\): void \{[\s\S]*?\n  \}\r?\n\r?\n  setAuthToken/,
        '\n  setAuthToken',
      )
      .replace(
        /  setAuthToken\(token: string\): void \{\r?\n    const headers = this\.getInternalHeaders\(\);\r?\n    if \(HttpClient\.API_KEY_HEADER\.toLowerCase\(\) !== 'authorization'\) \{\r?\n      delete headers\[HttpClient\.API_KEY_HEADER\];\r?\n    \}\r?\n    super\.setAuthToken\(token\);\r?\n  \}/,
        '  setAuthToken(token: string): void {\n    super.setAuthToken(token);\n  }',
      );
    writeTextIfChanged(httpClientSourcePath, source);
  }

  const commonTypesSourcePath = path.join(outputDir, 'src', 'types', 'common.ts');
  if (existsSync(commonTypesSourcePath)) {
    const source = readText(commonTypesSourcePath).replace(/\n  apiKey\?: string;\r?\n/g, '\n');
    writeTextIfChanged(commonTypesSourcePath, source);
  }

  const readmePath = path.join(outputDir, 'README.md');
  writeTextIfChanged(readmePath, renderGeneratedTypeScriptReadme(config));
}

function normalizeJavaAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'java');
  if (!existsSync(outputDir)) {
    return;
  }
  for (const filePath of walkFiles(outputDir)) {
    if (path.basename(filePath) === `${config.primaryClient}.java`) {
      const source = readText(filePath).replace(
        new RegExp(`\\n    public ${escapeRegExp(config.primaryClient)} setApiKey\\(String apiKey\\) \\{\\r?\\n        httpClient\\.setApiKey\\(apiKey\\);\\r?\\n        return this;\\r?\\n    \\}\\r?\\n`, 'g'),
        '\n',
      );
      writeTextIfChanged(filePath, source);
      continue;
    }
    if (path.basename(filePath) === 'HttpClient.java') {
      let source = readText(filePath).replace(
        /    private static final String API_KEY_HEADER = "(?:Authorization|Access-Token)";\r?\n    private static final boolean API_KEY_USE_BEARER = (?:true|false);\r?\n\r?\n/,
        '',
      );
      source = replaceBetween(
        source,
        /\n    public void setApiKey\(String apiKey\) \{\r?\n/,
        /\n    public void setHeader\(String key, String value\) \{\r?\n/,
        '\n    public void setAuthToken(String token) {\n        headers.put("Authorization", "Bearer " + token);\n    }\n\n    public void setAccessToken(String token) {\n        headers.put("Access-Token", token);\n    }\n\n',
        'Java generated auth methods',
      );
      writeTextIfChanged(filePath, source);
    }
  }
  normalizeGenericReadme(root, config, 'java');
}

function normalizeKotlinAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'kotlin');
  if (!existsSync(outputDir)) {
    return;
  }
  for (const filePath of walkFiles(outputDir)) {
    if (path.basename(filePath) === `${config.primaryClient}.kt`) {
      const source = readText(filePath).replace(
        new RegExp(`\\n    fun setApiKey\\(apiKey: String\\): ${escapeRegExp(config.primaryClient)} \\{\\r?\\n        httpClient\\.setApiKey\\(apiKey\\)\\r?\\n        return this\\r?\\n    \\}\\r?\\n`, 'g'),
        '\n',
      );
      writeTextIfChanged(filePath, source);
      continue;
    }
    if (path.basename(filePath) === 'HttpClient.kt') {
      let source = readText(filePath).replace(
        /    companion object \{\r?\n        private const val API_KEY_HEADER = "(?:Authorization|Access-Token)"\r?\n        private const val API_KEY_USE_BEARER = (?:true|false)\r?\n    \}\r?\n\r?\n/,
        '',
      );
      source = replaceBetween(
        source,
        /\n    fun setApiKey\(apiKey: String\) \{\r?\n/,
        /\n    fun setHeader\(key: String, value: String\) \{\r?\n/,
        '\n    fun setAuthToken(token: String) {\n        headers["Authorization"] = "Bearer $token"\n    }\n\n    fun setAccessToken(token: String) {\n        headers["Access-Token"] = token\n    }\n\n',
        'Kotlin generated auth methods',
      );
      writeTextIfChanged(filePath, source);
    }
  }
  normalizeGenericReadme(root, config, 'kotlin');
}

function normalizeRustAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'rust');
  if (!existsSync(outputDir)) {
    return;
  }
  const clientPath = path.join(outputDir, 'src', 'client.rs');
  if (existsSync(clientPath)) {
    const source = readText(clientPath).replace(
      /\n    pub fn set_api_key\(&self, api_key: impl Into<String>\) -> &Self \{\r?\n        self\.http\.set_api_key\(api_key\);\r?\n        self\r?\n    \}\r?\n/g,
      '\n',
    );
    writeTextIfChanged(clientPath, source);
  }
  const httpClientPath = path.join(outputDir, 'src', 'http', 'client.rs');
  if (existsSync(httpClientPath)) {
    let source = readText(httpClientPath).replace(
      /const DEFAULT_API_KEY_HEADER: &str = "(?:Authorization|Access-Token)";\r?\nconst DEFAULT_API_KEY_USE_BEARER: bool = (?:true|false);\r?\n\r?\n/,
      '',
    );
    source = replaceBetween(
      source,
      /\n    pub fn set_api_key\(&self, api_key: impl Into<String>\) \{\r?\n/,
      /\n    pub fn set_header\(&self, key: impl Into<String>, value: impl Into<String>\) \{\r?\n/,
      '\n    pub fn set_auth_token(&self, token: impl Into<String>) {\n        let mut headers = self.headers.write().expect("sdk headers poisoned");\n        headers.insert("Authorization".to_string(), format!("Bearer {}", token.into()));\n    }\n\n    pub fn set_access_token(&self, token: impl Into<String>) {\n        let mut headers = self.headers.write().expect("sdk headers poisoned");\n        headers.insert("Access-Token".to_string(), token.into());\n    }\n\n',
      'Rust generated auth methods',
    );
    writeTextIfChanged(httpClientPath, source);
  }
  normalizeGenericReadme(root, config, 'rust');
}

function normalizeGoAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'go');
  if (!existsSync(outputDir)) {
    return;
  }
  const sdkPath = path.join(outputDir, 'sdk.go');
  if (existsSync(sdkPath)) {
    const source = readText(sdkPath).replace(
      new RegExp(`\\nfunc \\(c \\*${escapeRegExp(config.primaryClient)}\\) SetApiKey\\(apiKey string\\) \\*${escapeRegExp(config.primaryClient)} \\{\\r?\\n    c\\.http\\.SetApiKey\\(apiKey\\)\\r?\\n    return c\\r?\\n\\}\\r?\\n`, 'g'),
      '\n',
    );
    writeTextIfChanged(sdkPath, source);
  }
  const httpClientPath = path.join(outputDir, 'http', 'client.go');
  if (existsSync(httpClientPath)) {
    let source = readText(httpClientPath)
      .replace(
        /const \(\r?\n    defaultApiKeyHeader = "(?:Authorization|Access-Token)"\r?\n    defaultApiKeyUseBearer = (?:true|false)\r?\n\)\r?\n\r?\n/,
        '',
      )
      .replace(
        /\/\/ Config wraps sdk-common Go config and adds SDK auth fields\.\r?\ntype Config struct \{\r?\n    common\.SdkConfig\r?\n    ApiKey      string\r?\n    AuthToken   string\r?\n    AccessToken string\r?\n\}\r?\n/,
        '// Config wraps sdk-common Go config and adds dual-token auth fields.\ntype Config struct {\n    common.SdkConfig\n    AuthToken   string\n    AccessToken string\n}\n',
      )
      .replace(
        /    if config\.ApiKey != "" \{\r?\n        client\.SetApiKey\(config\.ApiKey\)\r?\n    \}\r?\n/,
        '',
      );
    source = replaceBetween(
      source,
      /\nfunc \(c \*Client\) SetApiKey\(apiKey string\) \{\r?\n/,
      /\nfunc \(c \*Client\) SetHeader\(key, value string\) \{\r?\n/,
      '\nfunc (c *Client) SetAuthToken(token string) {\n    c.headers["Authorization"] = "Bearer " + token\n}\n\nfunc (c *Client) SetAccessToken(token string) {\n    c.headers["Access-Token"] = token\n}\n\n',
      'Go generated auth methods',
    );
    writeTextIfChanged(httpClientPath, source);
  }
  normalizeGenericReadme(root, config, 'go');
}

function pythonPackageDir(config) {
  return config.packages.python.replace(/-/g, '_');
}

function normalizePythonAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'python');
  const packageDir = path.join(outputDir, pythonPackageDir(config));
  if (!existsSync(packageDir)) {
    return;
  }
  const clientPath = path.join(packageDir, 'client.py');
  if (existsSync(clientPath)) {
    const source = readText(clientPath).replace(
      /\n    def set_api_key\(self, api_key: str\) -> '[^']+':\r?\n        """Set API key for authentication\."""\r?\n        self\._client\.set_api_key\(api_key\)\r?\n        return self\r?\n/g,
      '\n',
    );
    writeTextIfChanged(clientPath, source);
  }
  const httpClientPath = path.join(packageDir, 'http_client.py');
  if (existsSync(httpClientPath)) {
    let source = readText(httpClientPath).replace(
      /API_KEY_HEADER = '(?:Authorization|Access-Token)'\r?\nAPI_KEY_USE_BEARER = (?:True|False)\r?\n\r?\n/,
      '',
    );
    source = replaceBetween(
      source,
      /\n    def _update_auth_headers\(self\) -> None:\r?\n/,
      /\n    def set_header\(self, key: str, value: str\) -> 'HttpClient':\r?\n/,
      '\n    def _update_auth_headers(self) -> None:\n        if self._session is None:\n            return\n\n        self._session.headers.pop(\'Authorization\', None)\n        self._session.headers.pop(\'Access-Token\', None)\n        self._session.headers.pop(\'X-API-Key\', None)\n\n        if self._auth_token:\n            self._session.headers[\'Authorization\'] = f\'Bearer {self._auth_token}\'\n        if self._access_token:\n            self._session.headers[\'Access-Token\'] = self._access_token\n\n    def set_auth_token(self, token: str) -> \'HttpClient\':\n        self._auth_token = token\n        self._update_auth_headers()\n        return self\n\n    def set_access_token(self, token: str) -> \'HttpClient\':\n        self._access_token = token\n        self._update_auth_headers()\n        return self\n\n',
      'Python generated auth methods',
    );
    source = source.replace(
      /    Auth headers:\r?\n    - api_key -> Authorization: Bearer \{api_key\}\r?\n    - auth_token -> Authorization: Bearer \{auth_token\}\r?\n    - access_token -> Access-Token: \{access_token\}\r?\n/,
      '    Auth headers:\n    - auth_token -> Authorization: Bearer {auth_token}\n    - access_token -> Access-Token: {access_token}\n',
    );
    writeTextIfChanged(httpClientPath, source);
  }
  normalizeGenericReadme(root, config, 'python');
}

function normalizeFlutterAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'flutter');
  if (!existsSync(outputDir)) {
    return;
  }
  for (const filePath of walkFiles(outputDir)) {
    if (!readText(filePath).includes(`class ${config.primaryClient}`)) {
      continue;
    }
    let source = readText(filePath)
      .replace(/\n    String\? apiKey,\r?\n/g, '\n')
      .replace(/\n    String apiKeyHeader = 'Access-Token',\r?\n/g, '\n')
      .replace(/\n    bool apiKeyAsBearer = false,\r?\n/g, '\n')
      .replace(/\n        apiKey: apiKey,\r?\n/g, '\n')
      .replace(/\n        apiKeyHeader: apiKeyHeader,\r?\n/g, '\n')
      .replace(/\n        apiKeyAsBearer: apiKeyAsBearer,\r?\n/g, '\n')
      .replace(/\n  void setApiKey\(String apiKey\) \{\r?\n    _httpClient\.setApiKey\(apiKey\);\r?\n  \}\r?\n/g, '\n');
    writeTextIfChanged(filePath, source);
  }
  normalizeGenericReadme(root, config, 'flutter');
}

function normalizeSwiftAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'swift');
  if (!existsSync(outputDir)) {
    return;
  }
  const clientPath = path.join(outputDir, 'Sources', `${config.primaryClient}.swift`);
  if (existsSync(clientPath)) {
    const source = readText(clientPath).replace(
      new RegExp(`\\n    public func setApiKey\\(_ apiKey: String\\) -> ${escapeRegExp(config.primaryClient)} \\{\\r?\\n        httpClient\\.setApiKey\\(apiKey\\)\\r?\\n        return self\\r?\\n    \\}\\r?\\n`, 'g'),
      '\n',
    );
    writeTextIfChanged(clientPath, source);
  }
  const httpClientPath = path.join(outputDir, 'Sources', 'HTTP', 'HttpClient.swift');
  if (existsSync(httpClientPath)) {
    let source = readText(httpClientPath).replace(
      /    private static let apiKeyHeader = "(?:Authorization|Access-Token)"\r?\n    private static let apiKeyUseBearer = (?:true|false)\r?\n\r?\n/,
      '',
    );
    source = replaceBetween(
      source,
      /\n    public func setApiKey\(_ apiKey: String\) \{\r?\n/,
      /\n    public func setHeader\(_ key: String, value: String\) \{\r?\n/,
      '\n    public func setAuthToken(_ token: String) {\n        headers["Authorization"] = "Bearer \\(token)"\n    }\n\n    public func setAccessToken(_ token: String) {\n        headers["Access-Token"] = token\n    }\n\n',
      'Swift generated auth methods',
    );
    writeTextIfChanged(httpClientPath, source);
  }
  normalizeGenericReadme(root, config, 'swift');
}

function normalizeCsharpAuthSurface(root, config) {
  const outputDir = languageOutputDir(root, config, 'csharp');
  if (!existsSync(outputDir)) {
    return;
  }
  const clientPath = path.join(outputDir, `${config.primaryClient}.cs`);
  if (existsSync(clientPath)) {
    const source = readText(clientPath).replace(
      new RegExp(`\\n        public ${escapeRegExp(config.primaryClient)} SetApiKey\\(string apiKey\\)\\r?\\n        \\{\\r?\\n            _httpClient\\.SetApiKey\\(apiKey\\);\\r?\\n            return this;\\r?\\n        \\}\\r?\\n`, 'g'),
      '\n',
    );
    writeTextIfChanged(clientPath, source);
  }
  const httpClientPath = path.join(outputDir, 'Http', 'HttpClient.cs');
  if (existsSync(httpClientPath)) {
    let source = readText(httpClientPath).replace(
      /        private const string ApiKeyHeader = "(?:Authorization|Access-Token)";\r?\n        private static readonly bool ApiKeyUseBearer = (?:true|false);\r?\n\r?\n/,
      '',
    );
    source = replaceBetween(
      source,
      /\n        public void SetApiKey\(string apiKey\)\r?\n        \{\r?\n/,
      /\n        public void SetHeader\(string key, string value\)\r?\n        \{\r?\n/,
      '\n        public void SetAuthToken(string token)\n        {\n            _client.DefaultRequestHeaders.Authorization =\n                new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", token);\n        }\n\n        public void SetAccessToken(string token)\n        {\n            if (_client.DefaultRequestHeaders.Contains("Access-Token"))\n            {\n                _client.DefaultRequestHeaders.Remove("Access-Token");\n            }\n            _client.DefaultRequestHeaders.TryAddWithoutValidation("Access-Token", token);\n        }\n\n',
      'C# generated auth methods',
    );
    writeTextIfChanged(httpClientPath, source);
  }
  normalizeGenericReadme(root, config, 'csharp');
}

function normalizeGeneratedLanguage(root, config, language) {
  normalizeGeneratedPackageMetadata(root, config, language);
  if (language === 'typescript') {
    normalizeGeneratedTypeScriptAuthSurface(root, config);
  }
  if (language === 'flutter') {
    normalizeFlutterAuthSurface(root, config);
  }
  if (language === 'rust') {
    normalizeRustAuthSurface(root, config);
  }
  if (language === 'java') {
    normalizeJavaAuthSurface(root, config);
  }
  if (language === 'csharp') {
    normalizeCsharpAuthSurface(root, config);
  }
  if (language === 'swift') {
    normalizeSwiftAuthSurface(root, config);
  }
  if (language === 'kotlin') {
    normalizeKotlinAuthSurface(root, config);
  }
  if (language === 'go') {
    normalizeGoAuthSurface(root, config);
  }
  if (language === 'python') {
    normalizePythonAuthSurface(root, config);
  }
}

function buildGeneratedLanguage(root, config, language) {
  if (language !== 'typescript') {
    return;
  }
  const outputDir = languageOutputDir(root, config, language);
  const buildScriptPath = path.join(outputDir, 'custom', 'build-runtime.mjs');
  if (!existsSync(buildScriptPath)) {
    fail(config.sdkName, `typescript generated package is missing ${path.relative(root, buildScriptPath)}`);
  }
  run(config.sdkName, 'node', [buildScriptPath], {
    cwd: outputDir,
    step: 'typescript:custom/build-runtime.mjs',
  });
}

function languageOutputDir(root, config, language) {
  return path.join(root, `${config.sdkName}-${language}`, 'generated', 'server-openapi');
}

function prepareGeneratedOutput(root, config, language) {
  const prepareScript = path.join(root, 'bin', 'prepare-generated-output.mjs');
  if (!existsSync(prepareScript)) {
    return;
  }

  run(config.sdkName, 'node', [
    prepareScript,
    '--language',
    language,
  ], { cwd: root, step: `prepare-generated-output:${language}` });
}

function manifestNameFor(config, language) {
  if (language === 'csharp') {
    return `${config.namespaces.csharp}.csproj`;
  }
  return manifestByLanguage[language];
}

function languagePackageName(config, language) {
  return config.packages[language];
}

function inputForLanguage(root, config, language) {
  if (language === 'flutter' && config.flutterDerivedSpec) {
    return path.join(root, config.flutterDerivedSpec);
  }
  return path.join(root, config.derivedSpec);
}

function generatorArgs(root, config, language, resolvedVersion, baseUrl) {
  const args = [
    'generate',
    '--input',
    inputForLanguage(root, config, language),
    '--output',
    languageOutputDir(root, config, language),
    '--name',
    config.sdkName,
    '--type',
    config.sdkType,
    '--language',
    language,
    '--base-url',
    baseUrl,
    '--api-prefix',
    config.apiPrefix,
    '--fixed-sdk-version',
    resolvedVersion,
    '--sdk-root',
    root,
    '--sdk-name',
    config.sdkName,
    '--package-name',
    languagePackageName(config, language),
    '--standard-profile',
    'sdkwork-v3',
  ];
  if (language === 'csharp') {
    args.push('--namespace', config.namespaces.csharp);
  }
  return args;
}

async function readAuthority(root, config) {
  const yaml = await loadGeneratorYaml(root);
  return loadOpenApiDocument({
    prefix: config.sdkName,
    filePath: path.join(root, config.authoritySpec),
    yaml,
  });
}

function verifyOpenApiDocument(config, document, sourceLabel, failures) {
  if (!String(document.openapi || '').startsWith('3.')) {
    failures.push(`${sourceLabel} must be OpenAPI 3.x`);
  }
  const paths = Object.keys(document.paths ?? {});
  if (paths.length === 0) {
    failures.push(`${sourceLabel} must define paths`);
  }
  for (const pathKey of paths) {
    if (!pathKey.startsWith(config.apiPrefix)) {
      failures.push(`${sourceLabel} path must start with ${config.apiPrefix}: ${pathKey}`);
    }
    for (const forbidden of config.forbiddenPathParts) {
      if (pathKey.includes(forbidden)) {
        failures.push(`${sourceLabel} path must not contain ${forbidden}: ${pathKey}`);
      }
    }
  }
  for (const required of config.requiredPaths) {
    if (!paths.includes(required)) {
      failures.push(`${sourceLabel} must include required path ${required}`);
    }
  }
  const securitySchemes = document.components?.securitySchemes ?? {};
  if (securitySchemes.AuthToken?.type !== 'http' || securitySchemes.AuthToken?.scheme !== 'bearer') {
    failures.push(`${sourceLabel} must define components.securitySchemes.AuthToken as HTTP bearer`);
  }
  if (
    securitySchemes.AccessToken?.type !== 'apiKey'
    || securitySchemes.AccessToken?.in !== 'header'
    || securitySchemes.AccessToken?.name !== 'Access-Token'
  ) {
    failures.push(`${sourceLabel} must define components.securitySchemes.AccessToken as Access-Token header`);
  }
  if (!document.components?.schemas?.ProblemDetail) {
    failures.push(`${sourceLabel} must define components.schemas.ProblemDetail`);
  }

  for (const [pathKey, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      const normalizedMethod = method.toLowerCase();
      if (!['get', 'put', 'post', 'delete', 'patch', 'options', 'head', 'trace'].includes(normalizedMethod)) {
        continue;
      }
      if (!operation.operationId || !String(operation.operationId).includes('.')) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must use dotted operationId`);
      }
      if (!Array.isArray(operation.tags) || operation.tags.length === 0) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must declare tags`);
      }
      if (!operation.summary) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must declare summary`);
      }
      const security = operation.security;
      const anonymous = Array.isArray(security) && security.length === 0;
      const dualToken = Array.isArray(security)
        && security.some((entry) => entry && 'AuthToken' in entry && 'AccessToken' in entry);
      if (!anonymous && !dualToken) {
        failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} must use dual token security or security: []`);
      }
      for (const [status, response] of Object.entries(operation.responses ?? {})) {
        const statusNumber = Number.parseInt(status, 10);
        if (Number.isFinite(statusNumber) && statusNumber >= 400) {
          if (!response?.content?.['application/problem+json']) {
            failures.push(`${sourceLabel} ${normalizedMethod.toUpperCase()} ${pathKey} response ${status} must include application/problem+json`);
          }
        }
      }
    }
  }
}

function verifyReadme(root, config, failures) {
  const readmePath = path.join(root, 'README.md');
  if (!existsSync(readmePath)) {
    failures.push('README.md is required');
    return;
  }
  const source = readText(readmePath);
  for (const marker of [config.sdkName, config.apiPrefix, config.schemaUrl]) {
    if (!source.includes(marker)) {
      failures.push(`README.md must mention ${marker}`);
    }
  }
}

function verifyAssembly(root, config, languages, failures) {
  const assemblyPath = path.join(root, '.sdkwork-assembly.json');
  if (!existsSync(assemblyPath)) {
    failures.push('.sdkwork-assembly.json is required');
    return;
  }
  const assembly = readJson(assemblyPath);
  if (assembly.workspace !== config.sdkName) {
    failures.push('.sdkwork-assembly.json workspace must match SDK name');
  }
  if (assembly.discoverySurface?.apiPrefix !== config.apiPrefix) {
    failures.push('.sdkwork-assembly.json discoverySurface.apiPrefix must match API prefix');
  }
  if (assembly.discoverySurface?.sdkTarget !== config.sdkTarget) {
    failures.push('.sdkwork-assembly.json discoverySurface.sdkTarget must match SDK target');
  }
  if (assembly.generationInputSpec !== config.derivedSpec) {
    failures.push(`.sdkwork-assembly.json generationInputSpec must be ${config.derivedSpec}`);
  }
  const entries = new Map((assembly.languages ?? []).map((entry) => [entry.language, entry]));
  for (const language of languages) {
    const entry = entries.get(language);
    if (!entry) {
      failures.push(`.sdkwork-assembly.json must list language ${language}`);
      continue;
    }
    const expectedWorkspace = `${config.sdkName}-${language}`;
    const expectedPath = `${expectedWorkspace}/generated/server-openapi`;
    if (entry.workspace !== expectedWorkspace) {
      failures.push(`.sdkwork-assembly.json ${language}.workspace must be ${expectedWorkspace}`);
    }
    if (entry.generatedPath !== expectedPath) {
      failures.push(`.sdkwork-assembly.json ${language}.generatedPath must be ${expectedPath}`);
    }
    const expectedDescription = generatedDescriptionFor(config, language);
    if (expectedDescription && entry.description !== expectedDescription) {
      failures.push(`.sdkwork-assembly.json ${language}.description must be ${expectedDescription}`);
    }
  }
}

function* walkFiles(root, relative = '') {
  const current = path.join(root, relative);
  for (const entry of readdirSync(current, { withFileTypes: true })) {
    const childRelative = path.join(relative, entry.name);
    const childPath = path.join(root, childRelative);
    if (entry.isDirectory()) {
      if (entry.name === 'node_modules' || entry.name === 'dist' || entry.name === '.git' || entry.name === '.sdkwork') {
        continue;
      }
      yield* walkFiles(root, childRelative);
      continue;
    }
    if (entry.isFile() && textExtensions.has(path.extname(entry.name).toLowerCase())) {
      yield childPath;
    }
  }
}

function verifyGeneratedLanguage(root, config, language, failures) {
  const outputDir = languageOutputDir(root, config, language);
  if (!existsSync(outputDir)) {
    failures.push(`${language} generated output is missing: ${outputDir}`);
    return;
  }
  const manifestName = manifestNameFor(config, language);
  const manifestPath = path.join(outputDir, manifestName);
  if (!existsSync(manifestPath)) {
    failures.push(`${language} generated manifest is missing: ${manifestPath}`);
  }
  if (language === 'typescript') {
    const sdkMetadataPath = path.join(outputDir, 'sdkwork-sdk.json');
    const sdkSourcePath = path.join(outputDir, 'src', 'sdk.ts');
    const httpClientSourcePath = path.join(outputDir, 'src', 'http', 'client.ts');
    const commonTypesSourcePath = path.join(outputDir, 'src', 'types', 'common.ts');
    const generatedReadmePath = path.join(outputDir, 'README.md');
    if (!existsSync(sdkMetadataPath)) {
      failures.push('typescript sdkwork-sdk.json is required');
    } else {
      const metadata = readJson(sdkMetadataPath);
      if (metadata.sdkType !== config.sdkType) {
        failures.push(`typescript sdkwork-sdk.json sdkType must be ${config.sdkType}`);
      }
    }
    if (!existsSync(sdkSourcePath) || !readText(sdkSourcePath).includes(`class ${config.primaryClient}`)) {
      failures.push(`typescript SDK must export ${config.primaryClient}`);
    }
    for (const [relativePath, filePath] of [
      ['src/sdk.ts', sdkSourcePath],
      ['src/http/client.ts', httpClientSourcePath],
      ['src/types/common.ts', commonTypesSourcePath],
      ['README.md', generatedReadmePath],
    ]) {
      if (!existsSync(filePath)) {
        failures.push(`typescript generated ${relativePath} is required`);
        continue;
      }
      const source = readText(filePath);
      for (const forbidden of forbiddenGeneratedAuthSurfaceText) {
        if (source.includes(forbidden)) {
          failures.push(`typescript generated ${relativePath} must not expose API-key auth debt: ${forbidden}`);
        }
      }
    }
    for (const relativePath of [
      'dist/index.js',
      'dist/index.cjs',
      'dist/index.d.ts',
      'dist/sdk.d.ts',
      'dist/http/client.d.ts',
      'dist/types/common.d.ts',
    ]) {
      const filePath = path.join(outputDir, relativePath);
      if (!existsSync(filePath)) {
        failures.push(`typescript generated ${relativePath} is required`);
        continue;
      }
      const source = readText(filePath);
      for (const forbidden of forbiddenGeneratedAuthSurfaceText) {
        if (source.includes(forbidden)) {
          failures.push(`typescript generated ${relativePath} must not expose API-key auth debt: ${forbidden}`);
        }
      }
    }
  }
  for (const filePath of walkFiles(outputDir)) {
    const source = readText(filePath);
    const relative = relativePath(root, filePath);
    for (const forbidden of forbiddenGeneratedAuthSurfaceText) {
      if (source.includes(forbidden)) {
        failures.push(`${relative} must not expose API-key auth debt: ${forbidden}`);
      }
    }
    for (const forbidden of config.forbiddenGeneratedText) {
      if (source.includes(forbidden)) {
        failures.push(`${relative} must not contain ${forbidden}`);
      }
    }
  }
}

export async function runGenerateSdkFamily(config, argv) {
  const root = path.resolve(path.dirname(fileURLToPath(config.importMetaUrl)), '..');
  const args = parseGenerateArgs(config.sdkName, argv);
  const languages = parseLanguages(config.sdkName, args.languages);
  const generatorRoot = resolveGeneratorRoot(root);
  const generatorScript = path.join(generatorRoot, 'bin', 'sdkgen.js');
  const resolveVersionScript = path.join(generatorRoot, 'bin', 'resolve-sdk-version.js');

  if (args.refreshLive) {
    const refreshArgs = [
      path.join(root, 'bin', 'refresh-live-openapi-source.mjs'),
      '--schema-url',
      args.schemaUrl || new URL(config.schemaUrl, args.baseUrl).toString(),
      '--output',
      path.join(root, config.authoritySpec),
    ];
    run(config.sdkName, 'node', refreshArgs, { cwd: root, step: 'refresh-live-openapi-source' });
  }

  run(config.sdkName, 'node', [
    path.join(root, 'bin', 'prepare-openapi-source.mjs'),
    '--base',
    path.join(root, config.authoritySpec),
    '--derived',
    path.join(root, config.derivedSpec),
    '--prefer-derived',
  ], { cwd: root, step: 'prepare-openapi-source' });

  if (config.flutterDerivedSpec) {
    run(config.sdkName, 'node', [
      path.join(root, 'bin', 'prepare-openapi-source.mjs'),
      '--base',
      path.join(root, config.authoritySpec),
      '--derived',
      path.join(root, config.flutterDerivedSpec),
      '--prefer-derived',
      '--target-language',
      'flutter',
    ], { cwd: root, step: 'prepare-openapi-source:flutter' });
  }

  const authority = await readAuthority(root, config);
  const fixedSdkVersion = args.fixedSdkVersion || authority.info?.version || '0.1.0';
  const resolvedVersion = run(config.sdkName, 'node', [
    resolveVersionScript,
    '--sdk-root',
    root,
    '--sdk-name',
    config.sdkName,
    '--sdk-type',
    config.sdkType,
    '--fixed-sdk-version',
    fixedSdkVersion,
    '--package-name',
    config.packages.typescript,
    '--no-sync-published-version',
  ], { cwd: root, step: 'resolve-sdk-version', capture: true });

  for (const language of languages) {
    prepareGeneratedOutput(root, config, language);
    run(config.sdkName, 'node', [
      generatorScript,
      ...generatorArgs(root, config, language, resolvedVersion, args.baseUrl),
    ], { cwd: root, step: `sdkgen:${language}` });
    normalizeGeneratedLanguage(root, config, language);
    buildGeneratedLanguage(root, config, language);
    const manifestPath = path.join(languageOutputDir(root, config, language), manifestNameFor(config, language));
    if (!existsSync(manifestPath)) {
      fail(config.sdkName, `sdkgen:${language} did not materialize ${manifestPath}`);
    }
  }

  normalizeAssemblyDescriptions(root, config);
  await runVerifySdkFamily(config, ['--language', languages.join(',')]);
}

export async function runVerifySdkFamily(config, argv) {
  const root = path.resolve(path.dirname(fileURLToPath(config.importMetaUrl)), '..');
  const args = parseVerifyArgs(config.sdkName, argv);
  const languages = parseLanguages(config.sdkName, args.languages);
  const yaml = await loadGeneratorYaml(root);
  const failures = [];
  const authority = loadOpenApiDocument({
    prefix: config.sdkName,
    filePath: path.join(root, config.authoritySpec),
    yaml,
  });
  verifyOpenApiDocument(config, authority, config.authoritySpec, failures);
  const derived = loadOpenApiDocument({
    prefix: config.sdkName,
    filePath: path.join(root, config.derivedSpec),
    yaml,
  });
  verifyOpenApiDocument(config, derived, config.derivedSpec, failures);
  if (config.flutterDerivedSpec) {
    const flutterDerived = loadOpenApiDocument({
      prefix: config.sdkName,
      filePath: path.join(root, config.flutterDerivedSpec),
      yaml,
    });
    verifyOpenApiDocument(config, flutterDerived, config.flutterDerivedSpec, failures);
    if (Object.keys(flutterDerived.paths ?? {}).some((pathKey) => pathKey.endsWith('/realtime/ws'))) {
      failures.push(`${config.flutterDerivedSpec} must not expose websocket transport to generated Flutter HTTP SDK`);
    }
  }
  verifyReadme(root, config, failures);
  verifyAssembly(root, config, languages, failures);
  for (const language of languages) {
    verifyGeneratedLanguage(root, config, language, failures);
  }
  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(`[${config.sdkName}] ${failure}`);
    }
    process.exit(1);
  }
  console.log(`[${config.sdkName}] SDK family verification passed.`);
}
