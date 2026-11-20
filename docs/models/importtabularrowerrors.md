# ImportTabularRowErrors


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `row` | `i64` | :heavy_check_mark: | 1-based position of the data row in the file, the header row and blank rows not counted. For `runsPer: serial`, the first row of that serial. |
| `serial_number` | `Option<String>` | :heavy_minus_sign: | Serial number read from the row, when it has one. |
| `error` | `String` | :heavy_check_mark: | Why this row created no run. |
