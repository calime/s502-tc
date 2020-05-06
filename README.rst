S502 Toolchain
==============

This toolchain targets the 6502 mircoprocessor and has the following goals:

1. Use an object file format so code may be organized by a linker script and
   reused in any executable.

2. Have the ability to export the final addresses of the symbols after linking.
   This would be useful for creating a statically-loaded library and sharing
   the symbols of that library to link into any other executable without
   linking in the entire library.

3. Provide a higher level language designed to use the 6502's resources effectively.

Full documentation of the project can be found `here <https://calime.github.io/s502-tc/index.html>`_.

Building
--------

This project requires rust nightly. Build with ``cargo build``.
``mkdoc.sh`` is written for use in linux but may be translated to
batch for use in windows.