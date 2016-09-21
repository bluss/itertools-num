
Itertools-num
=============

Extra iterator adaptors, functions and macros.

Please read the `API documentation here`__

__ https://bluss.github.io/rust-itertools/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/rust-itertools.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/rust-itertools

.. |crates| image:: http://meritbadge.herokuapp.com/itertools
.. _crates: https://crates.io/crates/itertools

How to use with cargo::

    [dependencies]
    itertools = "0.5"

How to use in your crate:

.. code:: rust

    #[macro_use] extern crate itertools;

    use itertools::Itertools;

How to contribute:

- Fix a bug or implement a new thing
- Include tests for your new feature, preferably a quickcheck test
- Make a Pull Request


Recent Changes
--------------

- Not yet released

License
-------

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
