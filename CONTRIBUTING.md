# Contributing

First off, thank you for considering contributing to Rust GitHub Template.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

One of the project goals is to be easy to understand so, especially for github
actions, try to keep things simple and to add comments whenever this is not
possible.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/rust-github/template/issues),
please check that it has not already been reported by searching for some related
keywords.

Try to use a clear title, and describe your problem with complete sentences.

## Pull requests

Try to do one pull request per change.

## GitHub Actions

When we have to mix GitHub actions variables with the `{{ }}` liquid syntax,
GitHub actions variables are written in the format
`{{ "{{ github.variable }}" }}` instead of `{{ github.variable }}`.

See [Continuous delivery](https://github.com/rust-github/template/blob/main/template/.github/workflows/cd.yml)
as an example.
