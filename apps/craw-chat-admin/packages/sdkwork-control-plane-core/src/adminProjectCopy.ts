type AdminProjectLabelRecord = {
  id: string;
  name: string;
};

export function resolveAdminProjectLabel(
  projectId: string,
  projects: readonly AdminProjectLabelRecord[],
) {
  const normalizedProjectId = projectId.trim();
  const matchedProject = projects.find((project) => project.id.trim() === normalizedProjectId);
  const projectName = matchedProject?.name.trim();

  if (projectName) {
    return projectName;
  }

  return 'Workspace environment under review';
}
