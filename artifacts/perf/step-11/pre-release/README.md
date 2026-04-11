# Step 11 Pre-Release Tier artifact root

- artifactRoot: `artifacts/perf/step-11/pre-release`
- gateTemplate: `tools/perf/step-11-pre-release-tier-gate.json`
- evidenceIndex: `pre-release-tier-evidence-index.json`
- schema: `../schemas/step-11-tier-evidence-index.schema.json`
- checksumManifestPath: `checksum-manifest.txt`
- artifactFileListPath: `artifact-file-list.txt`
- current gate state: `evidence_collected_gate_blocked`
- collected slots: `connection_metrics`, `message_metrics`, `stream_metrics`, `drain_rebalance_drill`, `failover_drill`, `restore_recovery_drill`, `upgrade_rollback_drill`
- current evidence slot states: `collected`
- default naming rule: `artifactPath = artifactRoot + "/" + suggestedRelativePath`
- collected artifacts: `connection/metrics.json`, `message/metrics.json`, `stream/metrics.json`, `drain-rebalance/drill.json`, `failover/drill.json`, `restore-recovery/drill.json`, `upgrade-rollback/drill.json`
- collected runIds: `step11-cp11-2-local-connection-2026-04-08-doc-capture`, `step11-cp11-2-local-message-2026-04-08-doc-capture`, `step11-cp11-2-local-stream-2026-04-08-doc-capture`, `step11-cp11-3-local-drain-rebalance-2026-04-08-doc-capture`, `step11-cp11-3-local-failover-2026-04-08-doc-capture`, `step11-cp11-3-local-restore-recovery-2026-04-08-doc-capture`, `step11-cp11-3-local-upgrade-rollback-2026-04-08-doc-capture`
- boundary: this root now carries all seven truthful local artifacts. `Pre-Release Tier` is `evidence_collected_gate_blocked`, not full gate sign-off, because the artifacts are backfilled from published CI Smoke Tier / local-minimal evidence while `Capacity Tier` still stays `template_only_pending_execution`.
