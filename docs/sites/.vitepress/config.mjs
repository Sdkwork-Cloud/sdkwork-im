import { defineConfig } from "vitepress";
import { apiReferenceSidebar } from "./api-reference-sidebar.mjs";

const nav = [
  { text: "Getting Started", link: "/getting-started/index" },
  { text: "Architecture", link: "/architecture/overview" },
  { text: "Features", link: "/features/index" },
  { text: "API Reference", link: "/api-reference/index" },
  { text: "SDK", link: "/sdk/index" },
  { text: "Deployment", link: "/deployment/index" },
  { text: "Reference", link: "/reference/cli-and-scripts" },
];

const sidebar = {
  "/getting-started/": [
    {
      text: "Getting Started",
      items: [
        { text: "Overview", link: "/getting-started/index" },
        { text: "Quick Start", link: "/getting-started/quick-start" },
      ],
    },
  ],
  "/architecture/": [
    {
      text: "Architecture",
      items: [
        { text: "Overview", link: "/architecture/overview" },
        { text: "Runtime Topology", link: "/architecture/runtime-topology" },
        { text: "Module Map", link: "/architecture/module-map" },
      ],
    },
  ],
  "/features/": [
    {
      text: "Features",
      items: [
        { text: "Overview", link: "/features/index" },
        { text: "Capability Matrix", link: "/features/capabilities" },
      ],
    },
  ],
  "/api-reference/": apiReferenceSidebar,
  "/sdk/": [
    {
      text: "SDK",
      items: [
        { text: "Overview", link: "/sdk/index" },
        { text: "App SDK", link: "/sdk/app-sdk" },
        { text: "Admin SDK", link: "/sdk/admin-sdk" },
        { text: "Management SDK", link: "/sdk/management-sdk" },
        { text: "Language Support", link: "/sdk/language-support" },
      ],
    },
  ],
  "/deployment/": [
    {
      text: "Deployment",
      items: [
        { text: "Overview", link: "/deployment/index" },
        { text: "Local Binary", link: "/deployment/local-binary" },
        { text: "Server Lifecycle", link: "/deployment/server-lifecycle" },
        { text: "Docker", link: "/deployment/docker" },
        { text: "Profiles and Environment", link: "/deployment/profiles-and-env" },
        { text: "Runtime Operations", link: "/deployment/runtime-operations" },
      ],
    },
  ],
  "/reference/": [
    {
      text: "Reference",
      items: [
        { text: "CLI and Scripts", link: "/reference/cli-and-scripts" },
        { text: "Runtime Directory", link: "/reference/runtime-directory" },
      ],
    },
  ],
};

export default defineConfig({
  lang: "en-US",
  title: "Craw Chat",
  description:
    "Open-source product documentation for Craw Chat, aligned to the currently implemented architecture, runtime behavior, APIs, SDKs, and deployment workflows.",
  cleanUrls: true,
  lastUpdated: true,
  head: [
    ["meta", { name: "theme-color", content: "#8f5b34" }],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:title", content: "Craw Chat Docs" }],
  ],
  themeConfig: {
    siteTitle: "Craw Chat",
    nav,
    sidebar,
    logo: {
      light: "/logo-light.svg",
      dark: "/logo-dark.svg",
    },
    search: {
      provider: "local",
    },
    outline: {
      level: [2, 3],
      label: "On This Page",
    },
    docFooter: {
      prev: "Previous page",
      next: "Next page",
    },
    lastUpdated: {
      text: "Last updated",
    },
    footer: {
      message:
        "This site documents only behavior that is implemented and verified in the current repository state.",
      copyright: "MIT",
    },
  },
});
