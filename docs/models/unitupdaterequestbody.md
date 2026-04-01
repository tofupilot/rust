# UnitUpdateRequestBody


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `new_serial_number` | `Option<String>` | :heavy_minus_sign: | New serial number for the unit. |
| `part_number` | `Option<String>` | :heavy_minus_sign: | New part number for the unit. |
| `revision_number` | `Option<String>` | :heavy_minus_sign: | New revision number for the unit. |
| `batch_number` | `NullableField<String>` | :heavy_minus_sign: | New batch number for the unit. Set to null to remove batch. |
| `attachments` | `Option<Vec<String>>` | :heavy_minus_sign: | Array of upload IDs to attach to the unit. |
