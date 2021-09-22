[![Cargo test suite](https://github.com/ezzieyguywuf/rusty-egosum/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/ezzieyguywuf/rusty-egosum/actions/workflows/unit-tests.yml)

Summary
-------

This is a small project to generate a valid `EGO_SUM` variable for use in gentoo
ebuild's which utilize the [go-module eclass][1].

This program should replicate the following shell one-liner:

```sh
cat go.sum | cut -d" " -f1,2 | awk '{print "\t\"" $0 "\""}'
```

While writing this in rust is admittedly way overkill, I am using this as an
oportunity to get more familiar with the language.

Usage
-----

Since it is replacing such a simple bash one-liner, the usage is rather
straightforward:

```sh
git clone https://github.com/ezzieyguywuf/rusty-egosum
# get a go.sum from somwhere, e.g. wget https://raw.githubusercontent.com/wakatime/wakatime-cli/v1.26.1/go.sum
cargo run
```

TODO
----

- [ ] Add validation for parsed string (is it a real go module?)
- [ ] Add output-to-file
- [ ] Allow input from address (i.e. fetch `go.sum`)

[1]: https://devmanual.gentoo.org/eclass-reference/go-module.eclass/index.html
