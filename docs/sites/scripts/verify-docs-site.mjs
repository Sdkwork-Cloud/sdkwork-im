import fs from "node:fs";
import path from "node:path";

const docsRoot = process.cwd();
const repoRoot = path.resolve(docsRoot, "..", "..");
const issues = [];

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function assertFileExists(relativePath, baseDir = docsRoot) {
  const absolutePath = path.join(baseDir, relativePath);
  if (!fs.existsSync(absolutePath)) {
    issues.push(`${relativePath}: missing file`);
    return null;
  }
  return absolutePath;
}

function assertContains(relativePath, expectedText, baseDir = docsRoot) {
  const absolutePath = assertFileExists(relativePath, baseDir);
  if (!absolutePath) {
    return;
  }

  const content = read(absolutePath);
  if (!content.includes(expectedText)) {
    issues.push(`${relativePath}: missing "${expectedText}"`);
  }
}

assertContains("reference/admin-storage-contract.md", "/api/admin/storage/providers");
assertContains(
  "reference/admin-storage-contract.md",
  "/api/admin/storage/config/tenants/{tenantId}",
);
assertContains("reference/admin-storage-contract.md", "whole-record override");
assertContains(
  "reference/admin-storage-contract.md",
  "SDKWORK_ADMIN_SANDBOX_STORAGE_FILE",
);
assertContains("reference/admin-storage-contract.md", "StorageConfigUpsertInput");
assertContains("reference/admin-storage-contract.md", "StorageSecretSummaryRecord");
assertContains(
  "reference/admin-storage-contract.md",
  "not a published control-plane OpenAPI surface yet",
);

assertContains(".vitepress/config.ts", "/reference/admin-storage-contract");
assertContains("index.md", "/reference/admin-storage-contract");
assertContains("architecture/storage-management.md", "/reference/admin-storage-contract");
assertContains("reference/cli-and-scripts.md", "npm run docs:verify");
assertContains("reference/cli-and-scripts.md", "sdkwork-craw-chat-sdk");
assertContains("reference/cli-and-scripts.md", "craw-chat-app.sdkgen.yaml");
assertContains("reference/cli-and-scripts.md", "craw-chat-app.flutter.sdkgen.yaml");
assertContains(
  "reference/cli-and-scripts.md",
  "node .\\sdks\\sdkwork-craw-chat-sdk\\bin\\verify-sdk.mjs",
);
assertContains("reference/cli-and-scripts.md", "sdkwork-craw-chat-sdk-admin");
assertContains("reference/cli-and-scripts.md", "fetch-openapi-source.mjs");
assertContains("reference/cli-and-scripts.md", "prepare-openapi-source.mjs");
assertContains(
  "reference/cli-and-scripts.md",
  "node .\\sdks\\sdkwork-craw-chat-sdk-admin\\bin\\verify-sdk.mjs --language typescript --language flutter",
);
assertContains("reference/cli-and-scripts.md", ".sdkwork-assembly.json");

assertContains(
  "apps/craw-chat-admin/README.md",
  "../../docs/sites/reference/admin-storage-contract.md",
  repoRoot,
);

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log(
  "Verified admin storage reference page, docs navigation links, CLI docs, and admin README alignment.",
);
