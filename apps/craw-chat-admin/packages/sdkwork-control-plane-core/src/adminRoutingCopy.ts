import { adminRouteManifest } from './routeManifest';

function normalizeText(value: string | null | undefined) {
  return value?.trim() ?? '';
}

export function resolveAdminRoutingDecisionLabel(
  routeKey: string | null | undefined,
  capability: string | null | undefined,
) {
  const normalizedRouteKey = normalizeText(routeKey);
  const routeMatch = adminRouteManifest.find((route) => route.key === normalizedRouteKey);
  if (routeMatch) {
    return routeMatch.label;
  }

  const normalizedCapability = normalizeText(capability);
  const capabilityMatch = adminRouteManifest.find((route) =>
    route.productModule.capabilityTags.includes(normalizedCapability)
  );
  if (capabilityMatch) {
    return capabilityMatch.label;
  }

  return 'Workflow action under review';
}
