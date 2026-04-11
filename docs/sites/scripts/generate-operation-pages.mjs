import fs from "node:fs";
import path from "node:path";
import {
  groupedPages,
  markdownPathFor,
  operationMarkdownPath,
} from "../.vitepress/api-reference-sidebar.mjs";

const repoRoot = process.cwd();
const operationsRoot = path.join(repoRoot, "api-reference", "operations");

function sourceGroupFor(pageLink) {
  for (const group of groupedPages) {
    for (const page of group.pages) {
      if (page.link === pageLink) {
        return { domain: group.text, page: page.text };
      }
    }
  }

  return { domain: "API Reference", page: pageLink };
}

function operationBlocks(pageLink) {
  const content = fs.readFileSync(markdownPathFor(pageLink), "utf8");
  const matches = [
    ...content.matchAll(
      /<a id="([^"]+)"><\/a>\s*<section class="api-op">\s*## `([^`]+)`\s*([\s\S]*?)<\/section>/g,
    ),
  ];

  return matches.map((match) => ({
    anchor: match[1],
    operationTitle: match[2],
    body: match[3].trim(),
  }));
}

function operationFileContent(pageLink, operationTitle, body) {
  const { domain, page } = sourceGroupFor(pageLink);
  const cleanBody = body.replace(/^\s+/, "");

  return `# \`${operationTitle}\`

<p class="api-page-intro">
  OpenAPI-style operation reference for <strong>${page}</strong> in the <strong>${domain}</strong>.
</p>

<div class="api-link-list">
  <a href="${pageLink}">Back to ${page}</a>
</div>

<section class="api-op api-op-single">

${cleanBody}

</section>
`;
}

fs.rmSync(operationsRoot, { recursive: true, force: true });

let generatedCount = 0;

for (const group of groupedPages) {
  for (const page of group.pages) {
    for (const operation of operationBlocks(page.link)) {
      const outputPath = operationMarkdownPath(page.link, operation.anchor);
      fs.mkdirSync(path.dirname(outputPath), { recursive: true });
      fs.writeFileSync(
        outputPath,
        operationFileContent(page.link, operation.operationTitle, operation.body),
      );
      generatedCount += 1;
    }
  }
}

console.log(`Generated ${generatedCount} operation pages.`);
