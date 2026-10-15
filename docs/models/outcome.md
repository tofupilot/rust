# Outcome

Result of the measurement validation, across the validators on the measurement, its aggregations and its axes. Use FAIL when any validator fails, including on an empty value. Otherwise use UNSET when the value is empty and has no validators, or when a validator could not run (for example a numeric limit on a string). Otherwise use PASS, including for a recorded value with no validators.

## Values

| Variant | Wire Value |
| --- | --- |
| `Outcome::Pass` | `PASS` |
| `Outcome::Fail` | `FAIL` |
| `Outcome::Unset` | `UNSET` |
