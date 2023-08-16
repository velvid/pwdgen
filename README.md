# A random password generator

## Overview
A command line tool to generate passwords. This tool has the flexibility of specifying a minimum amount of a particular character pool (i.e. alphabet, numeric, or special).

## Usage
```
Usage: pwdgen.exe [OPTIONS]

Options:
  -l, --length <LENGTH>        Length of the password. Will be overriden if less than the sum of minimum characters. [default: 16]
  -a, --alpha <MIN_ALPHA>      Minimum alphabet characters. [default: 1]
  -n, --numeric <MIN_NUMERIC>  Minimum numeric characters. [default: 1]
  -s, --special <MIN_SPECIAL>  Minimum special characters. [default: 1]
  -h, --help                   Print help
```

## Notes

With no arguments provided, the default password length is 16, with at least one of each character pool guaranteed. If you would like to generate a password with only alphabet characters, you may do so with this:

```
pwdgen --length 0 --alpha 10 --numeric 0 --special 0
```

Since the length is 0, and only the minimum alphabet characters are stated, only alphabet characters are generated.
