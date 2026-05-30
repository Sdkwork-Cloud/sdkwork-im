export interface RouteMigrationResult {
  migratedRouteCount: number;
  sourceDrainStatus: string;
  sourceNodeId: string;
  sourceRebalanceState: string;
  targetDrainStatus: string;
  targetNodeId: string;
  targetRebalanceState: string;
}
