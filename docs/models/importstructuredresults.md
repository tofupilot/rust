# ImportStructuredResults


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `upload_id` | `String` | :heavy_check_mark: | Upload ID this result corresponds to. |
| `success` | `bool` | :heavy_check_mark: | Whether the file was imported successfully. |
| `id` | `Option<String>` | :heavy_minus_sign: | ID of the run (present when success is true): the run created, or the existing run when the file was already imported. For a multi-run file this is one of its runs, see `ids`. |
| `ids` | `Option<Vec<String>>` | :heavy_minus_sign: | All run ids of the file: the runs created, or the existing runs when the file was already imported. Present when there is more than one. |
| `already_imported` | `Option<bool>` | :heavy_minus_sign: | Present when success is true. True when the same file content was already imported in your organization and at least one of its runs still exists: `id` and `ids` are those runs and nothing was created. A file whose earlier import created only some of its runs is not completed: delete its runs, then import it again. For a file with several runs, `ids` comes in no guaranteed order, so `id` may name another run of that file than the first import returned. |
| `warnings` | `Option<Vec<String>>` | :heavy_minus_sign: | Non-fatal notes about this file, present when there are any: runs that failed while the others were imported, and how to create them later. They come only with the import that created the runs: a retry of the same file answers without them, so a client that lost that response should compare `ids` with the units it expects. |
| `error` | `Option<String>` | :heavy_minus_sign: | Error message (present when success is false). |
