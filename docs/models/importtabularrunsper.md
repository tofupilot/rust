# ImportTabularRunsPer

How rows become runs. `file` (default): one run for the whole file, which must hold a single serial number. `row`: one run per row, for a log that appends a row per test (retests of a serial give several runs); needs the wide or none measurement layout. `serial`: one run per distinct serial number, its rows grouped together.

## Values

| Variant | Wire Value |
| --- | --- |
| `ImportTabularRunsPer::File` | `file` |
| `ImportTabularRunsPer::Row` | `row` |
| `ImportTabularRunsPer::Serial` | `serial` |
