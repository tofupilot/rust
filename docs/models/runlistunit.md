# RunListUnit

Unit under test information.

## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unit ID. |
| `serial_number` | `String` | :heavy_check_mark: | Unit serial number. |
| `part` | `RunListPart` | :heavy_check_mark: | Part information with revision details. |
| `batch` | `NullableField<RunListBatch>` | :heavy_minus_sign: | Batch information for this unit. |
