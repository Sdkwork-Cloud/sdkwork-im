export async function bootstrapShellRuntime() {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-shell-app', 'sdkwork-craw-chat-admin');
  }

  await Promise.resolve();
}
