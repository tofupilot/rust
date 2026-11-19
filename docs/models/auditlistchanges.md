# AuditListChanges


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `field` | `String` | :heavy_check_mark: | N/A |
| `previous` | `Option<serde_json::Value>` | :heavy_minus_sign: | Value before the action. |
| `next` | `Option<serde_json::Value>` | :heavy_minus_sign: | Value after the action. |
