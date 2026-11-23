# Imports

## Overview

### Available Operations

* [structured](#structured) - Import runs from structured files
* [tabular](#tabular) - Import runs from a tabular file

## structured

Import one or more previously uploaded structured files (OpenHTF, WATS WSJF/WSXF, ATML, NI TestStand, STDF, or ATDF) in a single call. Each file is parsed independently and its result returned per-item, so one bad file does not fail the others. A file that contains several units (a multi-part STDF/ATDF datalog or a multi-report WSXF/TestStand document) creates one run per unit, and all run ids are returned in the item’s `ids`. A file whose exact content was already imported in your organization is not imported twice: its result returns the existing runs with `already_imported: true`, so retrying a batch is safe. A file whose earlier import created only some of its runs is not completed: the `warnings` of that first import say how to create the rest. Set `allow_duplicate` to import the file again as new runs, next to the existing ones.

### Example Usage

```rust
use tofupilot::TofuPilot;
use tofupilot::types::*;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.imports().structured()
        .items(vec![])
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `items` | `Vec<ImportStructuredItems>` | :heavy_check_mark: | Files to import (1–100). Pass a single-item list to import one file. Each item is parsed independently; one failure does not abort the others. |
| `allow_duplicate` | `Option<bool>` | :heavy_minus_sign: | Import every file even if the same content was already imported. By default, a file whose exact content already has a run in your organization is not imported again: its result returns the existing run with `already_imported: true`. Set to true to import such files again as new runs, next to the existing ones. |

### Response

**[`ImportStructuredResponse`](../../models/importstructuredresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## tabular

Import a previously uploaded tabular file (CSV or Excel) by mapping its columns to TofuPilot fields. Provide exactly one of `mapping` (an inline column mapping) or `template_id` (a mapping template saved in the dashboard). The `procedure_id` is required and overrides any procedure referenced in the file. By default the whole file is one run and must hold a single serial number. Set `mapping.runsPer` to `row` for a log with one run per row (retests included), or to `serial` to group the rows of each serial into one run: each row is mapped and created on its own, so a bad row is reported in `row_errors` and the other rows are imported. A request whose every row fails is refused with the rows named; send `run_offset` set to the end of the refused range to go on, or the same file and mapping again once the cause is fixed elsewhere (sub-units imported first): the requests already imported are skipped. One request creates at most 100 runs: while `next_run_offset` is in the response, send the same request with `run_offset` set to it. A request is not imported twice: a retry with the same file, `run_offset`, mapping and procedure returns the existing runs with `already_imported: true`, unless `allow_duplicate` is set. Timestamps without an offset, and dates alone, are read in `mapping.timeZone` (UTC by default). `mapping.subUnitColumns` links sub-units that already exist.

### Example Usage

```rust
use tofupilot::TofuPilot;
use tofupilot::types::*;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.imports().tabular()
        .upload_id("550e8400-e29b-41d4-a716-446655440000")
        .procedure_id("550e8400-e29b-41d4-a716-446655440000")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `upload_id` | `String` | :heavy_check_mark: | ID of a previously uploaded tabular file. |
| `procedure_id` | `String` | :heavy_check_mark: | Procedure to attach the imported run to. Always overrides any procedure referenced in the file. Create the procedure in the app first, then find the auto-generated ID on the procedure page. |
| `mapping` | `Option<ImportTabularMapping>` | :heavy_minus_sign: | Inline column mapping describing how source columns feed TofuPilot fields. Provide this OR template_id, not both. |
| `template_id` | `Option<String>` | :heavy_minus_sign: | ID of a saved mapping template to apply. Provide this OR mapping, not both. |
| `run_offset` | `Option<i64>` | :heavy_minus_sign: | With a mapping that makes several runs (`runsPer` `row` or `serial`): the position, in the file’s run order, of the first run to import. One request creates at most 100 runs; when runs remain, the response carries `next_run_offset`: send the same request again with it until it is absent. Defaults to 0. |
| `allow_duplicate` | `Option<bool>` | :heavy_minus_sign: | With a mapping that makes several runs (`runsPer` `row` or `serial`): create every run of the file, even those an earlier import created. By default a run an earlier import into the same procedure created is skipped and listed in `skipped_rows`, whatever the mapping and even when rows were appended to the file since: a run is recognized by its cells, or by the mapping’s `runKeyColumns`. Retrying a request is therefore safe, and so is importing a log again after it grew. A one-run-per-file mapping is never deduplicated. |

### Response

**[`ImportTabularResponse`](../../models/importtabularresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

