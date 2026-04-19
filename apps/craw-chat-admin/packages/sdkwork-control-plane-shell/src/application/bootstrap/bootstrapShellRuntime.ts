export async function bootstrapShellRuntime() {
  if (typeof document !== 'undefined') {
    document.documentElement.setAttribute('data-shell-app', 'sdkwork-control-plane');
  }

  await Promise.resolve();
}
