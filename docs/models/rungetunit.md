# RunGetUnit

Unit under test information.

## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unit ID. |
| `serial_number` | `String` | :heavy_check_mark: | Unit serial number. |
| `part` | `RunGetPart` | :heavy_check_mark: | Part information with revision details. |
| `batch` | `NullableField<RunGetBatch>` | :heavy_minus_sign: | Batch information for this unit. |
