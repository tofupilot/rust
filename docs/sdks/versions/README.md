# Versions

## Overview

### Available Operations

* [get](#get) - Get procedure version
* [delete](#delete) - Delete procedure version
* [create](#create) - Create procedure version

## get

Retrieve a single procedure version by its tag, including version metadata and configuration details.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.versions().get()
        .procedure_id("550e8400-e29b-41d4-a716-446655440000")
        .tag("value")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `procedure_id` | `String` | :heavy_check_mark: | ID of the procedure that owns this version. |
| `tag` | `String` | :heavy_check_mark: | Version tag to retrieve. |

### Response

**[`ProcedureGetVersionResponse`](../../models/proceduregetversionresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## delete

Permanently delete a procedure version by its tag. This removes the version record and all associated configuration data and cannot be undone.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.versions().delete()
        .procedure_id("550e8400-e29b-41d4-a716-446655440000")
        .tag("value")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `procedure_id` | `String` | :heavy_check_mark: | ID of the procedure that owns this version |
| `tag` | `String` | :heavy_check_mark: | Version tag to delete |

### Response

**[`ProcedureDeleteVersionResponse`](../../models/proceduredeleteversionresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::Conflict` | 409 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## create

Create a new version for an existing test procedure. Versions let you track procedure changes over time and maintain a history of test configurations.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.versions().create()
        .procedure_id("550e8400-e29b-41d4-a716-446655440000")
        .tag("value")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `procedure_id` | `String` | :heavy_check_mark: | The ID of the procedure this version belongs to |
| `tag` | `String` | :heavy_check_mark: | The version tag |

### Response

**[`ProcedureCreateVersionResponse`](../../models/procedurecreateversionresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::Conflict` | 409 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

