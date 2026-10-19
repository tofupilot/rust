# RunCreateSequences


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `path` | `String` | :heavy_check_mark: | Path of the sequence call: its instance key, prefixed by the keys of the calls it is nested in, joined by `/`. Unique within the run. A path with several segments requires its parent path (everything before the last `/`) to be present in the same array. |
| `name` | `String` | :heavy_check_mark: | Display name of the sequence call, as declared on the call or in the sequence file. |
| `source` | `NullableField<String>` | :heavy_minus_sign: | The sequence reference as written in the procedure (`uses:`), kept for display. |
| `outcome` | `SequencesOutcome` | :heavy_check_mark: | Outcome of the sequence call, aggregated over its phases like a run outcome: ERROR, then ABORTED, then TIMEOUT, then FAIL, then PASS. SKIP when none of its phases ran. |
| `started_at` | `NullableField<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | ISO 8601 timestamp when the first phase of the call started. Omit for a skipped call. |
| `ended_at` | `NullableField<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | ISO 8601 timestamp when the last phase of the call ended. Omit for a skipped call. |
