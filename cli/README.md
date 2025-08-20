# Unit conversion CLI Tool

Simple binary to usr the main library.

## Example Usage

### List available units

To get the list of all supported units:
```sh
runit_conversion list
```

### Conversion

Conversion is meant to be miminal without pretty print by default to be easily embeded in scripts.
By default when errors occurs return flag is set to 255, else 0 + print result

**Conversion kg -> g**
To convert Kilogram to gram use:
```sh
runit_conversion convert 50 kg^1 g^1
```


**Conversion kg/m3 -> g/cm3**
To convert density kg/m3 to  density in g/m3:
```sh
runit_conversion convert 50 kg^1*m^-3 g^1*cm^-3
```


**Conversion Pa/h -> g/mm/h²/h**
To convert more complex unit just
```sh
runit_conversion convert 50 Pa^1*h^-1 g^1*mm^-1*h^-3
```
