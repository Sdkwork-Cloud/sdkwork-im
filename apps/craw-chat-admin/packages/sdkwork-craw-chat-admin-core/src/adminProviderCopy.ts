type AdminProviderLabelRecord = {
  display_name: string;
  id: string;
};

export function resolveAdminProviderLabel(
  providerId: string,
  providers: readonly AdminProviderLabelRecord[],
) {
  const normalizedProviderId = providerId.trim();
  const matchedProvider = providers.find((provider) => provider.id.trim() === normalizedProviderId);
  const displayName = matchedProvider?.display_name.trim();

  if (displayName) {
    return displayName;
  }

  return 'Provider route under review';
}
