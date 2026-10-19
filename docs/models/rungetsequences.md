# RunGetSequences


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | N/A |
| `path` | `String` | :heavy_check_mark: | Instance keys of the call and of the calls it is nested in, joined by `/`. Unique within the run. |
| `key` | `String` | :heavy_check_mark: | The call's own instance key (last segment of `path`). |
| `parent_path` | `Option<String>` | :heavy_minus_sign: | Path of the enclosing call, null for a call made in the procedure itself. |
| `name` | `String` | :heavy_check_mark: | N/A |
| `source` | `Option<String>` | :heavy_minus_sign: | The sequence reference as written in the procedure (`uses:`). |
| `outcome` | `SequencesOutcome` | :heavy_check_mark: | Outcome of the call, aggregated over its phases like a run outcome. SKIP when none of them ran. |
| `started_at` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `ended_at` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `duration` | `Option<String>` | :heavy_minus_sign: | ISO 8601 duration, null for a skipped call. |
