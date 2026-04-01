# Procedures

## Overview

### Available Operations

* [list](#list) - List and filter procedures
* [create](#create) - Create procedure
* [get](#get) - Get procedure
* [delete](#delete) - Delete procedure
* [update](#update) - Update procedure

## list

Retrieve procedures with optional filtering and search. Returns all procedure data including creator, recent runs, and FPY (First Pass Yield) statistics.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.procedures().list()
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | `Option<i64>` | :heavy_minus_sign: | Maximum number of procedures to return per page. |
| `cursor` | `Option<f64>` | :heavy_minus_sign: | N/A |
| `search_query` | `Option<String>` | :heavy_minus_sign: | N/A |
| `created_after` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `created_before` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |

### Response

**[`ProcedureListResponse`](../../models/procedurelistresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## create

Create a new test procedure that can be used to organize and track test runs. The procedure serves as a template or framework for organizing test execution.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.procedures().create()
        .name("My Test Procedure")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `name` | `String` | :heavy_check_mark: | Name of the procedure. Must be unique within the organization. |

### Response

**[`ProcedureCreateResponse`](../../models/procedurecreateresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## get

Retrieve a single procedure by ID, including recent test runs, linked stations, and version history.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.procedures().get()
        .id("550e8400-e29b-41d4-a716-446655440000")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unique identifier of the procedure to retrieve. |

### Response

**[`ProcedureGetResponse`](../../models/proceduregetresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## delete

Permanently delete a procedure, removing all associated runs, phases, measurements, and attachments.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.procedures().delete()
        .id("550e8400-e29b-41d4-a716-446655440000")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unique identifier of the procedure to delete. |

### Response

**[`ProcedureDeleteResponse`](../../models/proceduredeleteresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## update

Update a test procedure's name or configuration. The procedure is identified by its unique ID in the URL path. Only provided fields are modified.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.procedures().update()
        .id("550e8400-e29b-41d4-a716-446655440000")
        .name("My Test Procedure")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `id` | `String` | :heavy_check_mark: | Unique identifier of the procedure to update. |
| `name` | `String` | :heavy_check_mark: | New name for the procedure. |

### Response

**[`ProcedureUpdateResponse`](../../models/procedureupdateresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

