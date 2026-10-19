# Identity

## Overview

### Available Operations

* [whoami](#whoami) - Who am I

## whoami

The identity the request is authenticated as: the station or the user behind the API key, and its organization. Works with both kinds of key; `type` says which one. Use it before a call that only one kind may make, instead of catching that call's refusal.

### Example Usage

```rust
use tofupilot::TofuPilot;

#[tokio::main]
async fn main() -> tofupilot::Result<()> {
    let client = TofuPilot::new("your-api-key");

    let result = client.identity().whoami()
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

**[`IdentityWhoamiResponse`](../../models/identitywhoamiresponse.md)**

### Errors

| Error Type | Status Code | Content Type |
| --- | --- | --- |
| `Error::Unauthorized` | 401 | application/json |
| `Error::NotFound` | 404 | application/json |
| `Error::UnexpectedStatus(408)` | 408 | application/json |
| `Error::InternalServerError` | 500 | application/json |
| `Error::UnexpectedStatus` | 4XX, 5XX | \*/\* |

