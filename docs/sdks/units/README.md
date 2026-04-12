# Units

## Overview

### Available Operations

* [list](#list) - List and filter units
* [create](#create) - Create unit
* [delete](#delete) - Delete units
* [get](#get) - Get unit
* [update](#update) - Update unit
* [add_child](#add_child) - Add sub-unit
* [remove_child](#remove_child) - Remove sub-unit
* [create_attachment](#create_attachment) - Attach file to unit
* [delete_attachment](#delete_attachment) - Delete unit attachments

## list

Retrieve a paginated list of units with filtering by serial number, part number, and batch. Uses cursor-based pagination for efficient large dataset traversal.

### Example Usage

```rust
use tofupilot::TofuPilot;
use tofupilot::types::*;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().list()
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `search_query` | `Option<String>` | :heavy_minus_sign: | N/A |
| `ids` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `serial_numbers` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `part_numbers` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `revision_numbers` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `batch_numbers` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `procedure_ids` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `outcomes` | `Option<Vec<Outcome>>` | :heavy_minus_sign: | N/A |
| `started_after` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `started_before` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `latest_only` | `Option<bool>` | :heavy_minus_sign: | N/A |
| `run_count_min` | `Option<i64>` | :heavy_minus_sign: | N/A |
| `run_count_max` | `Option<i64>` | :heavy_minus_sign: | N/A |
| `created_after` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `created_before` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | N/A |
| `created_by_user_ids` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `created_by_station_ids` | `Option<Vec<String>>` | :heavy_minus_sign: | N/A |
| `exclude_units_with_parent` | `Option<bool>` | :heavy_minus_sign: | N/A |
| `limit` | `Option<i64>` | :heavy_minus_sign: | Maximum number of units to return. |
| `cursor` | `Option<i64>` | :heavy_minus_sign: | N/A |
| `sort_by` | `Option<UnitListSortBy>` | :heavy_minus_sign: | Field to sort results by. last_run_at sorts by most recent test run date. last_run_procedure sorts by procedure name of the last run. |
| `sort_order` | `Option<ListSortOrder>` | :heavy_minus_sign: | Sort order direction. |

### Response

**[`UnitListResponse`](../../models/unitlistresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## create

Create a new unit with a serial number and link it to a part revision. Units represent individual hardware items tracked for manufacturing traceability.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().create()
        .serial_number("SN-001234")
        .part_number("PCB-V1.2")
        .revision_number("REV-A")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Unique serial number identifier for the unit. Must be unique within the organization. |
| `part_number` | `String` | :heavy_check_mark: | Component part number that defines what type of unit this is. If the part does not exist, it will be created. |
| `revision_number` | `String` | :heavy_check_mark: | Hardware revision identifier for the specific version of the part. If the revision does not exist, it will be created. |

### Response

**[`UnitCreateResponse`](../../models/unitcreateresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::Conflict` | 409 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## delete

Permanently delete units by serial number. This action will remove all nested elements and relationships associated with the units.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().delete()
        .serial_numbers(vec!["SN-001234".into()])
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_numbers` | `Vec<String>` | :heavy_check_mark: | Array of unit serial numbers to delete. |

### Response

**[`UnitDeleteResponse`](../../models/unitdeleteresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## get

Retrieve a single unit by its serial number. Returns comprehensive unit data including part information, parent/child relationships, and test run history.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().get()
        .serial_number("SN-001234")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Serial number of the unit to retrieve. |

### Response

**[`UnitGetResponse`](../../models/unitgetresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## update

Update unit properties including serial number, part revision, batch assignment, and file attachments with case-insensitive matching.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().update()
        .serial_number("SN-001234")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Serial number of the unit to update. |
| `new_serial_number` | `Option<String>` | :heavy_minus_sign: | New serial number for the unit. |
| `part_number` | `Option<String>` | :heavy_minus_sign: | New part number for the unit. |
| `revision_number` | `Option<String>` | :heavy_minus_sign: | New revision number for the unit. |
| `batch_number` | `NullableField<String>` | :heavy_minus_sign: | New batch number for the unit. Set to null to remove batch. |
| `attachments` | `Option<Vec<String>>` | :heavy_minus_sign: | Array of upload IDs to attach to the unit. |

### Response

**[`UnitUpdateResponse`](../../models/unitupdateresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::Conflict` | 409 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## add_child

Add a sub-unit to a parent unit to track component assemblies and multi-level hardware traceability.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().add_child()
        .serial_number("SN-001234")
        .child_serial_number("SN-001234")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Serial number of the parent unit |
| `child_serial_number` | `String` | :heavy_check_mark: | Serial number of the sub-unit to add |

### Response

**[`UnitAddChildResponse`](../../models/unitaddchildresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## remove_child

Remove a sub-unit relationship from a parent unit by serial number. Only unlinks the parent-child relationship; neither unit is deleted from the system.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().remove_child()
        .serial_number("SN-001234")
        .child_serial_number("SN-001234")
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Serial number of the parent unit |
| `child_serial_number` | `String` | :heavy_check_mark: | Serial number of the sub-unit to remove |

### Response

**[`UnitRemoveChildResponse`](../../models/unitremovechildresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::BadRequest` | 400 | application/json |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## create_attachment

Create an attachment linked to a unit and get a temporary pre-signed URL. Upload the file to the URL with a PUT request to complete the attachment.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().create_attachment()
        .serial_number("SN-001234")
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
| `serial_number` | `String` | :heavy_check_mark: | Serial number of the unit to attach the file to. Matched case-insensitively. |
| `name` | `String` | :heavy_check_mark: | File name including extension (e.g. "calibration.pdf"). Used to determine content type and display name. |

### Response

**[`UnitCreateAttachmentResponse`](../../models/unitcreateattachmentresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

## delete_attachment

Delete attachments from a unit by their IDs. Removes the files from storage and unlinks them from the unit.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.units().delete_attachment()
        .serial_number("SN-001234")
        .ids(vec!["550e8400-e29b-41d4-a716-446655440000".into()])
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `serial_number` | `String` | :heavy_check_mark: | Serial number of the unit. Matched case-insensitively. |
| `ids` | `Vec<String>` | :heavy_check_mark: | Attachment IDs to delete |

### Response

**[`UnitDeleteAttachmentResponse`](../../models/unitdeleteattachmentresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

