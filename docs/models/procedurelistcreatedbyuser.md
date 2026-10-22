# ProcedureListCreatedByUser

User who created the procedure.

## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unique identifier for the user who created the procedure. |
| `name` | `Option<String>` | :heavy_minus_sign: | Name of the user who created the procedure. |
| `email` | `Option<String>` | :heavy_minus_sign: | Email of the user who created the procedure. Null when the caller is a viewer restricted to access groups. |
