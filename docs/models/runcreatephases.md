# RunCreatePhases


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `String` | :heavy_check_mark: | Name identifier for the test phase. Each phase should have a descriptive name that identifies the specific stage of testing being performed. Analytics at phase level are computed using this name as unique identifier. |
| `outcome` | `PhasesOutcome` | :heavy_check_mark: | Overall result of the phase execution. Use PASS when phase succeeds, FAIL when phase fails but execution completed successfully, ERROR when phase execution fails, SKIP when phase was not executed. |
| `started_at` | `chrono::DateTime<chrono::Utc>` | :heavy_check_mark: | ISO 8601 timestamp when the phase execution began. |
| `ended_at` | `chrono::DateTime<chrono::Utc>` | :heavy_check_mark: | ISO 8601 timestamp when the phase execution completed. |
| `docstring` | `NullableField<String>` | :heavy_minus_sign: | N/A |
| `measurements` | `NullableField<Vec<RunCreateMeasurements>>` | :heavy_minus_sign: | N/A |
| `retry_count` | `Option<i64>` | :heavy_minus_sign: | Zero-based retry attempt index for this phase. 0 = first attempt, 1 = first retry, etc. When a phase is retried, all attempts are stored with the same name and increasing retry_count. |
