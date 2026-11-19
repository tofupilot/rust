# RunCreatePlugs


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `key` | `String` | :heavy_check_mark: | Key of the plug in the procedure: the parameter name a phase receives it under. Unique within the run. |
| `name` | `String` | :heavy_check_mark: | Display name of the plug, as declared in the procedure. |
| `python` | `NullableField<String>` | :heavy_minus_sign: | Class reference of the plug as written in the procedure (`python:`). |
| `scope` | `NullableField<Scope>` | :heavy_minus_sign: | Lifetime of the plug instance: one per slot, one per execution, or held by the station across runs. |
| `manufacturer` | `NullableField<String>` | :heavy_minus_sign: | Maker of the instrument the plug opened, the first field of its *IDN? answer. Omit when the plug reported none. |
| `model` | `NullableField<String>` | :heavy_minus_sign: | Model of the instrument, the second field of *IDN?. |
| `serial_number` | `NullableField<String>` | :heavy_minus_sign: | Serial number of the instrument, the third field of *IDN?. |
| `firmware` | `NullableField<String>` | :heavy_minus_sign: | Firmware version of the instrument, the fourth field of *IDN?. |
