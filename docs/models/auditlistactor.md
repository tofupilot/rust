# AuditListActor


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `r#type` | `AuditListType` | :heavy_check_mark: | Who acted. |
| `id` | `Option<String>` | :heavy_minus_sign: | ID of the user or station. Null for the system. |
| `name` | `Option<String>` | :heavy_minus_sign: | Name of the actor at the time of the event. |
| `email` | `Option<String>` | :heavy_minus_sign: | Email of the user at the time of the event. |
