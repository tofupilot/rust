# ImportTabularSerialNumber1


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `column` | `String` | :heavy_check_mark: | N/A |
| `extract` | `Option<String>` | :heavy_minus_sign: | Regular expression applied to the cell before it is used: the first capture group is kept, or the whole match when the pattern has no group. A cell the pattern does not match is used as written. For example `(\S+)$` keeps the last word of `LINE 2 TRAY 16S2P 2026C13-TR-001`. |
