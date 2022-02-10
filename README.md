# rekordcrate - Library for parsing Pioneer Rekordbox device exports

*rekordcrate* is library to parse device exports for the CDJ/XDJ series players
(usually exported from the Pioneer Rekordbox DJ software), written in Rust.

**Note:** This library is currently still under heavy development and might
have breaking API changes in the future.

## Command Line Usage

This library includes a command line tool named `rekordcrate-pdb` to inspect
database exports (i.e. `PIONEER/rekordbox/export.pdb` files):

    $ cargo run --bin rekordcrate-pdb data/demo-tracks/PIONEER/rekordbox/export.pdb

Analysis files (`.DAT`, `.EXT` and `.2EX` files in the `PIONEER/USBANLZ`
directory) can be viewed using the `rekordcrate-anlz` binary:

    $ cargo run --bin rekordcrate-anlz -- data/demo-tracks/PIONEER/USBANLZ/P016/0000875E/ANLZ0000.DAT

## FAQ

### Is this software affiliated with Pioneer Corp. or its related companies?

No, this library has been written independently.

## License

This software is licensed under the terms of the [Mozilla Public License
2.0](https://www.mozilla.org/en-US/MPL/2.0/). Please also have a look at the
[license FAQ](https://www.mozilla.org/en-US/MPL/2.0/FAQ/).