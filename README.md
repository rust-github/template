{%comment%}
# Github Rust template

## New projects

Install cargo-generate:

```shell
cargo install cargo-generate
```

Generate a new project:

```shell
cargo generate --git https://github.com/Yoshanuikabundi/rust-template.git -b main
```

## README template begins here
-----
{%endcomment%}

{{"#"}} {{project-name}}

[![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/{{username}}/{{project-name}}?label=tag&logo=github&sort=semver)](https://github.com/{{username}}/{{project-name}})
[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})
[![CI](https://github.com/{{username}}/{{project-name}}/workflows/Continuous%20Integration/badge.svg)](https://github.com/{{username}}/{{project-name}}/actions)
[![Coverage Status](https://coveralls.io/repos/github/{{username}}/{{project-name}}/badge.svg?branch=main)](https://coveralls.io/github/{{username}}/{{project-name}}?branch=main)
[![MIT License](https://img.shields.io/github/license/{{username}}/{{project-name}})](https://github.com/{{username}}/{{project-name}}/blob/main/LICENSE-MIT)
[![APACHE License](https://img.shields.io/github/license/{{username}}/{{project-name}})](https://github.com/{{username}}/{{project-name}}/blob/main/LICENSE-APACHE)

## License

{{project-name}} is licensed under your choice of the Apache or MIT licenses. See the LICENSE-MIT and LICENSE-APACHE files in the project's root directory.