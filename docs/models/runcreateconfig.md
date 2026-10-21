# RunCreateConfig


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `key` | `String` | :heavy_check_mark: | Key as declared under `config:` or `secrets:` in procedure.yaml. |
| `value` | `serde_json::Value` | :heavy_check_mark: | Value the run used, in its declared type. |
| `variable_id` | `NullableField<String>` | :heavy_minus_sign: | ID of the stored value the station received, when it came from TofuPilot. |
| `version` | `NullableField<i64>` | :heavy_minus_sign: | Version of the stored value the station received. |
| `source` | `NullableField<Source>` | :heavy_minus_sign: | Where the value came from: the YAML `default:`, the shell environment, a `.env` file, the station process environment, or a value stored on the station, the procedure or the organization. |
