# RunGetPlugs


## Fields

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `key` | `String` | :heavy_check_mark: | Key of the plug in the procedure. |
| `name` | `String` | :heavy_check_mark: | Display name of the plug. |
| `python` | `Option<String>` | :heavy_minus_sign: | Class reference of the plug as written in the procedure. |
| `scope` | `Option<Scope>` | :heavy_minus_sign: | Lifetime of the plug instance: slot, execution or station. |
| `manufacturer` | `Option<String>` | :heavy_minus_sign: | Maker of the instrument. |
| `model` | `Option<String>` | :heavy_minus_sign: | Model of the instrument. |
| `serial_number` | `Option<String>` | :heavy_minus_sign: | Serial number of the instrument. |
| `firmware` | `Option<String>` | :heavy_minus_sign: | Firmware version of the instrument. |
