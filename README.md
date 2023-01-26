# libmm

> **Warning**
> This library is now at active development stage, so it may be incomplete,
> may contain errors and bugs, etc.
>
> Main repository: [mm][1] (there you can find some project documentation).

This library implements a backend for `mm` notes app.
If contains the following components:

- [ ] repositories
- [ ] configuration
- [ ] data container abstraction

## Build

If you want to build library for `mm` itself, so you have to read *Build* section 
in the [main repository][1] readme.

Building for custom frontend:

1. Add library to you Cargo.toml:

```toml
[dependencies]
libmm = { path = "/path/to/libmm" }
```

2. Build your main project.

[1]: https://github.com/GeorgyFirsov/mm
