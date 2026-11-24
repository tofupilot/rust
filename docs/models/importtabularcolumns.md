# ImportTabularColumns


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `column` | `String` | :heavy_check_mark: | N/A |
| `name` | `Option<String>` | :heavy_minus_sign: | N/A |
| `units` | `Option<String>` | :heavy_minus_sign: | N/A |
| `lower_limit` | `Option<f64>` | :heavy_minus_sign: | N/A |
| `upper_limit` | `Option<f64>` | :heavy_minus_sign: | N/A |
| `phase` | `Option<String>` | :heavy_minus_sign: | N/A |
| `outcome_map` | `Option<std::collections::HashMap<String, serde_json::Value>>` | :heavy_minus_sign: | Turns the column into a step result: each cell value maps to the measurement outcome (`PASS`, `FAIL`, `UNSET`), or to `null` to leave that cell out, such as `NOT_RUN`. A failed measurement fails its phase. A value with no entry fails the row. |
