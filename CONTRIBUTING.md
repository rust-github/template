# Contribution guidelines

First off, thank you for considering contributing to {{project-name}}.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Issues have to be reported on our [[https://github.com/{{username}}/{{project-name}}/issues][issues tracker]]. Please:

- Check that the issue has not already been reported.
  - This can be achieved by searching keywords on the [[https://github.com/{{username}}/{{project-name}}/issues][issues tracker]].
- Try to use a clear title, and describe your problem with complete sentences.
  - If it is a bug report, try to include details on how to reproduce it, like
    a step by step guide.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/{{username}}/{{project-name}}/blob/master/CHANGELOG.org)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by [[https://keepachangelog.com/en/1.0.0/][Keep a changelog]]:

- Added :: for new features.
- Changed :: for changes in existing functionality.
- Deprecated :: for soon-to-be removed features.
- Removed :: for now removed features.
- Fixed :: for any bug fixes.
- Security :: in case of vulnerabilities.

If the required subsection does not exist yet under =Unreleased=, create it!
