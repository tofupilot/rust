# Outcome

Result of the measurement validation, across the validators on the measurement, its aggregations and its axes. Use FAIL when any validator fails, including on an empty value. Otherwise use UNSET when there are no validators or one could not run (for example a numeric limit on a string). Otherwise use PASS.

## Values

| Variant | Wire Value |
| --- | --- |
| `Outcome::Pass` | `PASS` |
| `Outcome::Fail` | `FAIL` |
| `Outcome::Unset` | `UNSET` |
