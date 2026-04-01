# User

## Overview

### Available Operations

* [list](#list) - List users

## list

Retrieve a list of users in your organization. Use the current parameter to get only the authenticated user profile and permissions.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.user().list()
        .send()
        .await?;

    println!("{:?}", result);
    Ok(())
}
```

### Parameters

| Parameter | Type | Required | Description |
| --- | --- | --- | --- |
| `current` | `Option<bool>` | :heavy_minus_sign: | If true, returns only the current authenticated user |

### Response

**`Vec<UserListResponse>`**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

