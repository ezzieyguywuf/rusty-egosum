This is a small project to generate a valid `EGO_SUM` variable for use in gentoo
ebuild's which utilize the [go-module eclass][1].

This program should replicate the following shell one-liner:

```sh
cat go.sum | cut -d" " -f1,2 | awk '{print "\t\"" $0 "\""}'
```

While writing this in rust is admittedly way overkill, I am using this as an
oportunity to get more familiar with the language.

[1]: https://devmanual.gentoo.org/eclass-reference/go-module.eclass/index.html