# dxf2elmt
dxf2elmt is CLI program which can convert .dxf files into [QElectroTech](https://qelectrotech.org/) .elmt files. The program supports both ascii and binary .dxf files.

The goal of this program is to create a fast and accurate conversion tool to be used with [QElectroTech](https://qelectrotech.org/).

## How to Use
dxf2elmt requires only one input from the user, the input file.

For example:

```bash
./dxf2elmt my_file.dxf
```

The .elmt file will be output into the same directory as the executable. It will retain the name of the .dxf file.

If you wish to forgo creating an .elmt file, you can use the "-v" argument for verbose output. This will output the contents of the .elmt file to stdout without actually creating the file. For example:

```bash
./dxf2elmt my_file.dxf -v
```

## Supported Entities

* Lines
* Circles
* Arcs
* Texts
* Ellipses
* Polylines
* LwPolylines
* Solids
* Splines
* Blocks
* MText (partial support)
* Leader

## To Do

* Support for the following
    * Remaining 2d entities

* Better error messages
* Logging

## Compiling

Compiled using Rust (MSRV 1.74.1).

## Credits

* [QElectroTech](https://qelectrotech.org/)
* [dxf-rs](https://github.com/IxMilia/dxf-rs)
* [simple-xml-builder](https://github.com/Accelbread/simple-xml-builder)
* [bspline](https://github.com/Twinklebear/bspline)
* [tempfile](https://github.com/Stebalien/tempfile)
