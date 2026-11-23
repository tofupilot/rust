# ImportTabularMapping

Inline column mapping describing how source columns feed TofuPilot fields. Provide this OR template_id, not both.

## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `fields` | `ImportTabularFields` | :heavy_check_mark: | N/A |
| `measurements` | `serde_json::Value` | :heavy_check_mark: | N/A |
| `phases` | `Option<ImportTabularPhases>` | :heavy_minus_sign: | N/A |
| `metadata` | `Option<Vec<ImportTabularMetadata>>` | :heavy_minus_sign: | N/A |
| `unit_metadata` | `Option<Vec<ImportTabularUnitMetadata>>` | :heavy_minus_sign: | N/A |
| `runs_per` | `Option<ImportTabularRunsPer>` | :heavy_minus_sign: | How rows become runs. `file` (default): one run for the whole file, which must hold a single serial number. `row`: one run per row, for a log that appends a row per test (retests of a serial give several runs); needs the wide or none measurement layout. `serial`: one run per distinct serial number, its rows grouped together. |
| `sub_unit_columns` | `Option<Vec<String>>` | :heavy_minus_sign: | Columns holding sub-unit serial numbers, linked to the run’s unit like `sub_units` on run creation. Empty cells are skipped. Each sub-unit must already exist: import its own runs first, or the row fails with `Sub-unit not found`. |
| `time_zone` | `Option<String>` | :heavy_minus_sign: | IANA time zone for timestamps without an offset, such as `2026-07-08T15:49:48.349659`, `2026-07-08 15:49:48` or the date-only `2025-03-10` (read as midnight). Defaults to UTC. Timestamps with an offset or `Z`, and epoch numbers, are read as written. |
| `ignore_missing_columns` | `Option<bool>` | :heavy_minus_sign: | Skip mapped columns the file does not have instead of failing the file, so one mapping covers files whose optional columns vary. The serial number, outcome and time columns, and the name and value columns of the long layout, stay required. |
