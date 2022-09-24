# Album Creator

A tool to process selected images.

## Usage

the tool can be used from the command line:

```bash
album-creator --conf config_file.json
```

## Parameters

| Name | Description |
| --- | --- |
| config | absolute path of the configuration file |

## Configuration file

### Simple example
```json
{
  "name": "test album",
  "base": "/home/user/picture",
  "images": [
    { "filename": "picture1.jpg" },
    { "filename": "picture2.jpg" },
    { "filename": "picture3.jpg" }
  ]
}
```

## License

This work is licensed under the MIT license.

`SPDX-License-Identifier: MIT`