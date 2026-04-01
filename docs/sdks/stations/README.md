# Stations

## Overview

### Available Operations

* [list](#list) - List and filter stations
* [create](#create) - Create station
* [get_current](#get_current) - Get current station
* [get](#get) - Get station
* [remove](#remove) - Remove station
* [update](#update) - Update station

## list

Retrieve a paginated list of test stations in your organization. Search by station name and filter by status for station fleet management.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.stations().list()
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `limit` | `Option<i64>` | :heavy_minus_sign: | Number of stations to return per page |
| `cursor` | `Option<i64>` | :heavy_minus_sign: | N/A |
| `search_query` | `Option<String>` | :heavy_minus_sign: | N/A |
| `procedure_ids` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |

### Response

**[`StationListResponse`](../../models/stationlistresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## create

Create a new test station in TofuPilot to register production equipment and link it to test procedures.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.stations().create()
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
| `name` | `String` | :heavy_check_mark: | Name of the station |
| `procedure_id` | `Option<String>` | :heavy_minus_sign: | Optional procedure ID to link the station to |

### Response

**[`StationCreateResponse`](../../models/stationcreateresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::Forbidden` | 403 | application/json |
| `Error::Conflict` | 409 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## get_current

Retrieve detailed information about the currently authenticated station including linked procedures and connection status.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.stations().get_current()
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |

### Response

**[`StationGetCurrentResponse`](../../models/stationgetcurrentresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::Forbidden` | 403 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## get

Retrieve detailed station information including linked procedures, connection status, and recent activity.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.stations().get()
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
| `id` | `String` | :heavy_check_mark: | Unique identifier of the station to retrieve |

### Response

**[`StationGetResponse`](../../models/stationgetresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## remove

Remove a test station. Deletes permanently if unused, or archives with preserved historical data if runs exist.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.stations().remove()
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
| `id` | `String` | :heavy_check_mark: | Unique identifier of the station to remove |

### Response

**[`StationRemoveResponse`](../../models/stationremoveresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## update

Update station name and/or image. The station ID is specified in the URL path. To remove an image, pass an empty string for image_id.

### Example Usage

```rust
use tofupilot::TofuPilotClient;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilotClient::new("your-api-key");

    let result = client.stations().update()
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
| `id` | `String` | :heavy_check_mark: | Unique identifier of the station to update |
| `name` | `Option<String>` | :heavy_minus_sign: | New name for the station |
| `image_id` | `Option<String>` | :heavy_minus_sign: | Upload ID for the station image, or empty string to remove image |
| `team_id` | `NullableField<String>` | :heavy_minus_sign: | Team ID to assign this station to, or null to unassign |

### Response

**[`StationUpdateResponse`](../../models/stationupdateresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::Conflict` | 409 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

