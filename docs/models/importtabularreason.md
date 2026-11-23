# ImportTabularReason

`already_imported`: the row is unchanged. `changed`: the `runKeyColumns` match an imported run but other cells differ, a row edited after its import. The existing run is kept as it is: delete it and import again to replace it.

## Values

| Variant | Wire Value |
| --- | --- |
| `ImportTabularReason::AlreadyImported` | `already_imported` |
| `ImportTabularReason::Changed` | `changed` |
