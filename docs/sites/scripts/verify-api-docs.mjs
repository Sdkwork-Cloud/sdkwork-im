import fs from "node:fs";
import path from "node:path";
import {
  apiReferenceOperationLinks,
  groupedPages,
  markdownPathFor,
  operationMarkdownPath,
  operationPageLink,
  readOperations,
} from "../.vitepress/api-reference-sidebar.mjs";

const repoRoot = process.cwd();
const issues = [];
const sourceOperationLinks = [];

const blockPattern =
  /<a id="([^"]+)"><\/a>[\s\S]*?<section class="api-op">([\s\S]*?)(?=<a id="|$)/g;

function verifySourceOperationBlock(filePath, anchor, block) {
  const titleMatch = block.match(/## `([^`]+)`/);
  const title = titleMatch?.[1] ?? anchor;
  const relativePath = path.relative(repoRoot, filePath).replaceAll("\\", "/");
  const location = `${relativePath}#${anchor}`;
  const metaMatch = block.match(/<div class="api-meta-grid">([\s\S]*?)<\/div>\s*(?=\n### )/);
  const route = title.replace(/^[A-Z]+\s+/, "");
  const isPost = title.startsWith("POST ");
  const hasPathParams = title.includes("{");
  const isOpenProbe = route === "/healthz" || route === "/readyz";

  if (!block.includes("operationId:")) {
    issues.push(`${location}: missing operationId`);
  }

  if (!/### Response `\d+`/.test(block)) {
    issues.push(`${location}: missing explicit response section`);
  }

  if (hasPathParams && !block.includes("### Path Parameters")) {
    issues.push(`${location}: missing path parameter table`);
  }

  if (
    isPost &&
    !block.includes("### Request Body") &&
    !/does not require a JSON request body/i.test(block) &&
    !/does not accept a JSON request body/i.test(block)
  ) {
    issues.push(`${location}: missing request body section or explicit no-body note`);
  }

  if (!metaMatch) {
    issues.push(`${location}: missing api-meta-grid`);
  } else {
    for (const label of ["Security", "SDK", "Permission", "Success"]) {
      if (!metaMatch[1].includes(`<strong>${label}</strong>`)) {
        issues.push(`${location}: missing api-meta-grid label "${label}"`);
      }
    }

    if (
      (relativePath.includes("docs/sites/api-reference/platform/") ||
        relativePath.includes("docs/sites/api-reference/iot/")) &&
      metaMatch[1].includes("`sdkwork-craw-chat-sdk`")
    ) {
      issues.push(
        `${location}: platform and IoT operation docs must not claim sdkwork-craw-chat-sdk as the SDK surface`,
      );
    }

    if (!isOpenProbe && metaMatch[1].includes("trusted headers")) {
      issues.push(
        `${location}: public Security metadata must describe the bearer-auth contract only; keep trusted-header details in shared auth docs or endpoint notes`,
      );
    }
  }

  if (!isOpenProbe && !block.includes("### Error Responses")) {
    issues.push(`${location}: missing error responses section`);
  }
}

for (const group of groupedPages) {
  for (const page of group.pages) {
    const filePath = markdownPathFor(page.link);
    if (!fs.existsSync(filePath)) {
      issues.push(`${page.link}: missing source overview page`);
      continue;
    }

    const content = fs.readFileSync(filePath, "utf8");
    let match;
    while ((match = blockPattern.exec(content))) {
      const [, anchor, block] = match;
      verifySourceOperationBlock(filePath, anchor, block);
      sourceOperationLinks.push(operationPageLink(page.link, anchor));
    }
  }
}

const expectedSidebarLinks = new Set(sourceOperationLinks);
const sidebarLinks = [...apiReferenceOperationLinks];
const sidebarLinkSet = new Set(sidebarLinks);

for (const link of sidebarLinks) {
  const markdownPath = path.join(repoRoot, `${link.replace(/^\//, "")}.md`);
  if (!fs.existsSync(markdownPath)) {
    issues.push(`sidebar ${link}: missing operation markdown page`);
    continue;
  }

  const markdownContent = fs.readFileSync(markdownPath, "utf8");
  if (!markdownContent.includes("<section class=\"api-op")) {
    issues.push(`sidebar ${link}: missing operation section wrapper`);
  }
  if (!/### Response `\d+`/.test(markdownContent)) {
    issues.push(`sidebar ${link}: missing explicit response section`);
  }
}

for (const link of expectedSidebarLinks) {
  if (!sidebarLinkSet.has(link)) {
    issues.push(`${link}: missing sidebar entry`);
  }
}

for (const group of groupedPages) {
  for (const page of group.pages) {
    for (const operation of readOperations(page.link)) {
      const operationPath = operationMarkdownPath(page.link, operation.anchor);
      if (!fs.existsSync(operationPath)) {
        issues.push(`${operationPath}: missing generated operation page`);
        continue;
      }

      const content = fs.readFileSync(operationPath, "utf8");
      if (!content.includes(`# \`${operation.operationTitle}\``)) {
        issues.push(`${operationPath}: missing operation title heading`);
      }
      if (!content.includes("Back to")) {
        issues.push(`${operationPath}: missing overview backlink`);
      }
    }
  }
}

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log(
  `Verified ${groupedPages.reduce((sum, group) => sum + group.pages.length, 0)} source API pages, ${sourceOperationLinks.length} operation pages, and ${sidebarLinks.length} sidebar entries.`,
);
