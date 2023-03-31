
A library for rustdocifying `README.md` for inclusion in `lib.rs`.

- Removes top-level header.
- Changes other headers to be one level higher.
- Converts package-internal `docs.rs` links to rustdoc format.
- Doesn't change anything within code blocks.
- (optional) Checks that converted links have correct version and crate name.
- No `unsafe`.
- No dependencies.

# Usage

- Add this to `Cargo.toml`:

```toml
[build-dependencies]
readme-rustdocifier = "0.1.0"
```

- Create `README.md`.
- Create `build.rs` with following content:

```no_run
use std::{env, error::Error, fs, path::PathBuf};

const CRATE_NAME: &str = "your_crate_name_here";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=README.md");
    fs::write(
        PathBuf::from(env::var("OUT_DIR")?).join("README-rustdocified.md"),
        readme_rustdocifier::rustdocify(
            &fs::read_to_string("README.md")?,
            &env::var("CARGO_PKG_NAME")?,
            Some(&env::var("CARGO_PKG_VERSION")?),
            Some(CRATE_NAME),
        )?,
    )?;
    Ok(())
}
```

- Add this to start of `lib.rs`:

```no_run
#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-rustdocified.md"))]
```

- Run `cargo doc` and see the generated documentation of your library.

# Example `README.md`

<!-- Note: Using extra `#`:s here because rustdoc removes one. -->
```markdown
## foo

A foo library.

### Usage

Create [`Foo::new`].

[`Foo::new`]: https://docs.rs/foo/*/foo/struct.Foo.html#method.new
```

Above `README.md` is rustdocified to:

<!-- Note: Using extra `#`:s here because rustdoc removes one. -->
```markdown

A foo library.

## Usage

Create [`Foo::new`].

[`Foo::new`]: crate::Foo::new
```

# Link conversions

Lines like `[...]: https://docs.rs/PACKAGE/...` are converted to rustdoc format.

Following conversions are done:

- `https://docs.rs/PACKAGE` (1)
    - `crate`
- `https://docs.rs/PACKAGE#fragment` (1)
    - `crate#fragment`
- `https://docs.rs/PACKAGE/VERSION` (2)
    - `crate`
- `https://docs.rs/PACKAGE/VERSION#fragment` (2)
    - `crate#fragment`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES` (2)
    - `crate::MODULES`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES#fragment` (2)
    - `crate::MODULES#fragment`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/enum.ENUM.html`
    - `crate::MODULES::ENUM`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/enum.ENUM.html#method.METHOD`
    - `crate::MODULES::ENUM::METHOD`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/enum.ENUM.html#variant.VARIANT`
    - `crate::MODULES::ENUM::VARIANT`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/enum.ENUM.html#fragment`
    - `crate::MODULES::ENUM#fragment`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/fn.FUNCTION.html`
    - `crate::MODULES::FUNCTION`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/fn.FUNCTION.html#fragment`
    - `crate::MODULES::FUNCTION#fragment`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/macro.MACRO.html`
    - `crate::MODULES::MACRO`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/macro.MACRO.html#fragment`
    - `crate::MODULES::MACRO#fragment`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/struct.STRUCT.html`
    - `crate::MODULES::STRUCT`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/struct.STRUCT.html#method.METHOD`
    - `crate::MODULES::STRUCT::METHOD`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/struct.STRUCT.html#fragment`
    - `crate::MODULES::STRUCT#fragment`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/trait.TRAIT.html`
    - `crate::MODULES::TRAIT`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/trait.TRAIT.html#tymethod.METHOD`
    - `crate::MODULES::TRAIT::METHOD`
- `https://docs.rs/PACKAGE/VERSION/CRATE/MODULES/trait.TRAIT.html#fragment`
    - `crate::MODULES::TRAIT#fragment`


Notes:

- (1) Can have optional `/` at path end.
- (2) Can have optional `/` or `/index.html` at path end.
- `/MODULES` and corresponding `::MODULES` can be empty.

# Safety

This crate doesn't use any `unsafe` code.
This is enforced by `#![forbid(unsafe_code)]`.
