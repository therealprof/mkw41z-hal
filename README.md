mkw41z-hal
==========

_mkw41z-hal_ contains the (slight) hardware abstraction on top of the
peripheral access API for the NXP KW41Z series microcontroller.

This crate relies on my [mkw41z][] crate to provide appropriate register
definitions and implements a partial set of the [embedded-hal][] traits.

This implementation was developped and tested against the [FRDM-KW41Z][] board
for which also a [FRDM-KW41Z crate][] is available.

[mkw41z]: https://github.com/therealprof/mkw41z.git
[embedded-hal]: https://github.com/japaric/embedded-hal.git
[FRDM-KW41Z]: https://www.nxp.com/products/processors-and-microcontrollers/arm-based-processors-and-mcus/kinetis-cortex-m-mcus/w-serieswireless-conn.m0-plus-m4/freedom-development-kit-for-kinetis-kw41z-31z-21z-mcus:FRDM-KW41Z
[FRDM-KW41Z crate]: https://github.com/therealprof/FRDM-KW41Z.git

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
