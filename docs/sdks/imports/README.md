# Imports

## Overview

### Available Operations

* [create_from_files](#create_from_files) - Import runs from files

## create_from_files

Import one or more previously uploaded files (OpenHTF, WATS WSJF/WSXF, ATML, NI TestStand, STDF, or ATDF) in a single call. Each file is parsed independently and its result returned per-item, so one bad file does not fail the others. A file that contains several units (a multi-part STDF/ATDF datalog or a multi-report WSXF/TestStand document) creates one run per unit; all run ids are returned in the item’s `ids`.

### Example Usage

```rust
use tofupilot::TofuPilot;
use tofupilot::types::*;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.imports().create_from_files()
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
| `items` | `Vec<ImportCreateFromFilesItems>` | :heavy_check_mark: | Files to import (1–100). Pass a single-item list to import one file. Each item is parsed independently; one failure does not abort the others. |

### Response

**[`ImportCreateFromFilesResponse`](../../models/importcreatefromfilesresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

