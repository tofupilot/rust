# ImportTabularResponse

Run imported successfully

## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | ID of the created run. With a mapping that makes several runs, the first one, see `ids`. |
| `file_import_id` | `String` | :heavy_check_mark: | ID of the file import record that links the runs to the uploaded file. |
| `ids` | `Option<Vec<String>>` | :heavy_minus_sign: | Present with a mapping that makes several runs (`runsPer` `row` or `serial`): every run id created by this request, in file order, rows that failed or were skipped left out. When `already_imported` is true, the existing runs. |
| `already_imported` | `Option<bool>` | :heavy_minus_sign: | Present with a mapping that makes several runs. True when every run of this request already existed (see `skipped_rows`): `id` and `ids` are those runs and nothing was created. |
| `row_errors` | `Option<Vec<ImportTabularRowErrors>>` | :heavy_minus_sign: | Present with a mapping that makes several runs: the rows of this request that created no run and why, at most 1000 (see `failed_row_count`). The other rows are imported. |
| `failed_row_count` | `Option<i64>` | :heavy_minus_sign: | Present with a mapping that makes several runs: how many runs of this request failed, including any not listed in `row_errors`. |
| `skipped_rows` | `Option<Vec<ImportTabularSkippedRows>>` | :heavy_minus_sign: | Present with a mapping that makes several runs: the rows of this request an earlier import already created, at most 1000 (see `skipped_row_count`). They create no run. |
| `skipped_row_count` | `Option<i64>` | :heavy_minus_sign: | Present with a mapping that makes several runs: how many runs of this request were skipped because an earlier import created them. |
| `total_runs` | `Option<i64>` | :heavy_minus_sign: | Present with a mapping that makes several runs: how many runs the whole file makes (rows for `runsPer: row`, distinct serials for `runsPer: serial`), failed ones included. |
| `next_run_offset` | `Option<i64>` | :heavy_minus_sign: | Present when runs of the file remain after this request: send the same request with `run_offset` set to this value to import the next ones. |
| `warnings` | `Option<Vec<String>>` | :heavy_minus_sign: | Non-fatal notes about the import, present when there are any. |
