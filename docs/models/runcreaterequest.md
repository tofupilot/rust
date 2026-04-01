# RunCreateRequest


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `outcome` | `Outcome` | :heavy_check_mark: | Overall test result. Use PASS when test succeeds, FAIL when test fails but script execution completed successfully, ERROR when script execution fails, TIMEOUT when test exceeds time limit, ABORTED for manual script interruption. |
| `procedure_id` | `String` | :heavy_check_mark: | Procedure ID. Create the procedure in the app first, then find the auto-generated ID on the procedure page. |
| `procedure_version` | `NullableField<String>` | :heavy_minus_sign: | N/A |
| `operated_by` | `Option<String>` | :heavy_minus_sign: | Email address of the operator who executed the test run. The operator must exist as a user in the system. The run will be linked to this user to track who performed the test. |
| `started_at` | `chrono::DateTime<chrono::Utc>` | :heavy_check_mark: | ISO 8601 timestamp when the test run began execution. This timestamp will be used to track when the test execution started and for historical analysis of test runs. A separate created_at timestamp is stored internally server side to track upload date. |
| `ended_at` | `chrono::DateTime<chrono::Utc>` | :heavy_check_mark: | ISO 8601 timestamp when the test run finished execution. |
| `serial_number` | `String` | :heavy_check_mark: | Unique serial number of the unit under test. Matched case-insensitively. If no unit with this serial number exists, one will be created. |
| `part_number` | `Option<String>` | :heavy_minus_sign: | Component part number for the unit. Matched case-insensitively. This field is required if the part number cannot be extracted from the serial number (as set in the settings). This field takes precedence over extraction from serial number. A component with the provided or extracted part number will be created if one does not exist. |
| `revision_number` | `Option<String>` | :heavy_minus_sign: | Hardware revision identifier for the unit. Matched case-insensitively. If none exist, a revision with this number will be created. If no revision is specified, the unit will be linked to the default revision of the part number. |
| `batch_number` | `Option<String>` | :heavy_minus_sign: | Production batch identifier for grouping units manufactured together. Matched case-insensitively. If none exist, a batch with this batch number will be created. If no batch number is specified, the unit will not be linked to any batch. |
| `sub_units` | `Option<Vec<String>>` | :heavy_minus_sign: | Array of sub-unit serial numbers that are part of this main unit. Matched case-insensitively. Each sub-unit must already exist and will be linked as a sub-component of the main unit under test. If no sub-units are specified, the unit will be created without sub-unit relationships. |
| `docstring` | `Option<String>` | :heavy_minus_sign: | Additional notes or documentation about this test run. |
| `phases` | `Option<Vec<RunCreatePhases>>` | :heavy_minus_sign: | Array of test phases with measurements and results. Each phase represents a distinct stage of the test execution with timing information, outcome status, and optional measurements. If no phases are specified, the run will be created without phase-level organization of test data. |
| `logs` | `Option<Vec<RunCreateLogs>>` | :heavy_minus_sign: | Array of log messages generated during the test execution. Each log entry captures events, errors, and diagnostic information with severity levels and source code references. If no logs are specified, the run will be created without log entries. |
