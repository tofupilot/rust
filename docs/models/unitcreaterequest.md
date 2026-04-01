# UnitCreateRequest


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Unique serial number identifier for the unit. Must be unique within the organization. |
| `part_number` | `String` | :heavy_check_mark: | Component part number that defines what type of unit this is. If the part does not exist, it will be created. |
| `revision_number` | `String` | :heavy_check_mark: | Hardware revision identifier for the specific version of the part. If the revision does not exist, it will be created. |
