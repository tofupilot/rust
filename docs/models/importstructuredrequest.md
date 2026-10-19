# ImportStructuredRequest


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | `Vec<ImportStructuredItems>` | :heavy_check_mark: | Files to import (1–100). Pass a single-item list to import one file. Each item is parsed independently; one failure does not abort the others. |
| `allow_duplicate` | `Option<bool>` | :heavy_minus_sign: | Import every file even if the same content was already imported. By default, a file whose exact content already has a run in your organization is not imported again: its result returns the existing run with `already_imported: true`. Set to true to import such files again as new runs, next to the existing ones. |
