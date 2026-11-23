# ImportTabularSkippedRows


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `row` | `i64` | :heavy_check_mark: | 1-based position of the data row in the file, as in `row_errors`. |
| `serial_number` | `Option<String>` | :heavy_minus_sign: | Serial number read from the row, when it has one. |
| `run_id` | `String` | :heavy_check_mark: | The run an earlier import created for this row. |
| `reason` | `ImportTabularReason` | :heavy_check_mark: | `already_imported`: the row is unchanged. `changed`: the `runKeyColumns` match an imported run but other cells differ, a row edited after its import. The existing run is kept as it is: delete it and import again to replace it. |
