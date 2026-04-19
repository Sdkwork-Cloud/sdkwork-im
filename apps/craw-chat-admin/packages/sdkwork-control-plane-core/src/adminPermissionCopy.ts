const ADMIN_PERMISSION_LABELS: Record<string, string> = {
  'admin.overview.read': 'Overview access',
  'admin.tenants.read': 'Tenant registry access',
  'admin.tenants.write': 'Tenant changes',
  'admin.users.read': 'Identity roster access',
  'admin.users.write': 'Identity changes',
  'admin.groups.read': 'Group directory access',
  'admin.groups.write': 'Group governance changes',
  'admin.announcements.read': 'Announcement access',
  'admin.announcements.write': 'Announcement changes',
  'admin.conversations.read': 'Conversation access',
  'admin.conversations.write': 'Conversation governance changes',
  'admin.messages.read': 'Transcript audit access',
  'admin.messages.moderate': 'Evidence and moderation actions',
  'admin.moderation.read': 'Moderation queue access',
  'admin.moderation.write': 'Moderation policy changes',
  'admin.automation.read': 'Automation access',
  'admin.automation.write': 'Automation changes',
  'admin.realtime.read': 'Realtime posture access',
  'admin.storage.read': 'Storage access',
  'admin.storage.write': 'Storage changes',
  'admin.system.read': 'System governance access',
  'admin.settings.read': 'Settings access',
  'admin.settings.write': 'Settings changes',
};

export function resolveAdminPermissionLabel(permission: string) {
  return ADMIN_PERMISSION_LABELS[permission] ?? permission;
}
