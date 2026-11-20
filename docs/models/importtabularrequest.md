# ImportTabularRequest


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `upload_id` | `String` | :heavy_check_mark: | ID of a previously uploaded tabular file. |
| `procedure_id` | `String` | :heavy_check_mark: | Procedure to attach the imported run to. Always overrides any procedure referenced in the file. Create the procedure in the app first, then find the auto-generated ID on the procedure page. |
| `mapping` | `Option<ImportTabularMapping>` | :heavy_minus_sign: | Inline column mapping describing how source columns feed TofuPilot fields. Provide this OR template_id, not both. |
| `template_id` | `Option<String>` | :heavy_minus_sign: | ID of a saved mapping template to apply. Provide this OR mapping, not both. |
| `run_offset` | `Option<i64>` | :heavy_minus_sign: | With a mapping that makes several runs (`runsPer` `row` or `serial`): the position, in the file’s run order, of the first run to import. One request creates at most 100 runs; when runs remain, the response carries `next_run_offset`: send the same request again with it until it is absent. Defaults to 0. |
| `allow_duplicate` | `Option<bool>` | :heavy_minus_sign: | With a mapping that makes several runs (`runsPer` `row` or `serial`): import the file even if the same content was already imported. By default a request with the same file content, `run_offset`, mapping and procedure as an earlier one whose runs still exist is not imported again: the response returns the existing runs with `already_imported: true`, so retrying a request is safe. A one-run-per-file mapping is never deduplicated. |
