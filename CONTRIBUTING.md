# Contribution guidelines

Thanks for considering contributing to {{project-name}}!

If you intend to provide a substantial contribution, please consider first creating an issue to discuss the changes with other maintainers.

## Pull requests

Before you submit a pull request, please consider these guidelines:

- Always follow the [code of conduct](https://www.rust-lang.org/conduct.html)
- Submit unrelated changes in separate pull requests
- Add a descriptive entry to the changelog (see below)
- Ensure your code...
  - passes all tests
  - is formatted according to `rustfmt`
  - compiles without warnings
  - passes the enabled `clippy` lints without warnings
  - is appropriately documented

### Updating the changelog

When submitting a pull request, please add an entry briefly describing your contribution to the **Unreleased** section of [CHANGELOG.md](https://github.com/yoshanuikabundi/{{project-name}}/blob/main/CHANGELOG.md). This helps us provide a detailed changelog to our users at release time. Please use the appropriate subsection:

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

