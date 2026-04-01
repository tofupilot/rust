# ProcedureListRuns


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unique identifier for the run. |
| `outcome` | `Outcome` | :heavy_check_mark: | Result of the test run. |
| `started_at` | `chrono::DateTime<chrono::Utc>` | :heavy_check_mark: | ISO 8601 timestamp when the run was started. |
| `unit` | `ProcedureListUnit` | :heavy_check_mark: | Unit associated with this run. |
