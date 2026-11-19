# AuditListRequest


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `occurred_after` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | Only events that occurred at or after this time (ISO 8601). |
| `occurred_before` | `Option<chrono::DateTime<chrono::Utc>>` | :heavy_minus_sign: | Only events that occurred at or before this time (ISO 8601). |
| `actions` | `Option<Vec<String>>` | :heavy_minus_sign: | Filter by action, written resource.verb. |
| `actor_ids` | `Option<Vec<String>>` | :heavy_minus_sign: | Filter by the user or station that acted. |
| `vias` | `Option<Vec<Via>>` | :heavy_minus_sign: | N/A |
| `outcomes` | `Option<Vec<DataOutcome>>` | :heavy_minus_sign: | N/A |
| `target_type` | `Option<String>` | :heavy_minus_sign: | Filter by the type of record acted on. |
| `target_id` | `Option<String>` | :heavy_minus_sign: | Filter by the ID of the record acted on. |
| `request_id` | `Option<String>` | :heavy_minus_sign: | Filter by request ID, to find the events one API request produced. |
| `search_query` | `Option<String>` | :heavy_minus_sign: | Match the actor name or email, the record name, or the action. |
| `limit` | `Option<i64>` | :heavy_minus_sign: | Maximum number of events to return. |
| `cursor` | `NullableField<serde_json::Value>` | :heavy_minus_sign: | Opaque cursor of the next page: pass next_cursor from the previous response unchanged. A keyset cursor rather than an offset: paging is stable while new events arrive. An event is timestamped when its request starts and appears when it commits, so a poller re-reads the last 10 minutes and de-duplicates by id. |
