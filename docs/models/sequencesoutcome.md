# SequencesOutcome

Outcome of the sequence call, aggregated over its phases like a run outcome: ERROR, then ABORTED, then TIMEOUT, then FAIL, then PASS. SKIP when none of its phases ran.

## Values

| Variant | Wire Value |
| --- | --- |
| `SequencesOutcome::Pass` | `PASS` |
| `SequencesOutcome::Fail` | `FAIL` |
| `SequencesOutcome::Error` | `ERROR` |
| `SequencesOutcome::Timeout` | `TIMEOUT` |
| `SequencesOutcome::Aborted` | `ABORTED` |
| `SequencesOutcome::Skip` | `SKIP` |
