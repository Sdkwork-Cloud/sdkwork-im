# Step 11 Capacity Tier artifact root

- artifactRoot: `artifacts/perf/step-11/capacity`
- gateTemplate: `tools/perf/step-11-capacity-tier-gate.json`
- evidenceIndex: `capacity-tier-evidence-index.json`
- schema: `../schemas/step-11-tier-evidence-index.schema.json`
- checksumManifestPath: `checksum-manifest.txt`
- artifactFileListPath: `artifact-file-list.txt`
- current gate state: `template_only_pending_execution`
- current evidence slot state: `pending_collection`
- default naming rule: `artifactPath = artifactRoot + "/" + suggestedRelativePath`
- boundary: this root is materialized so operators have a stable drop target before real `Capacity Tier` evidence is collected.
