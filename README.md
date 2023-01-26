# libmm

> **Warning**
> This library is now at active development stage, so it may be incomplete,
> may contain errors and bugs, etc.
>
> Main repository: [mm][1] (there you can find some project documentation).

This library implements a backend for `mm` notes app.
It contains the following components:

- [ ] repositories
- [ ] configuration
- [ ] data container abstraction

## Build

If you want to build library for `mm` itself, so you have to read [Build][2] section
in the [main repository][1] readme.

Building for custom frontend:

1. Clone repository recursively (i.e. with submodules):

```bash
git clone --recursive https://github.com/GeorgyFirsov/libmm.git
```

2. Add library to you Cargo.toml:

```toml
[dependencies]
libmm = { path = "/path/to/libmm" }
```

3. Build your main project.

[1]: https://github.com/GeorgyFirsov/mm
[2]: https://github.com/GeorgyFirsov/mm#build
