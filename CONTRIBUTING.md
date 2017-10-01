Contributor Guidelines
===

Thank you for contributing to Cirs! Please have a read through this document
and make sure you understand it all before contributing.

# Pull Requests

If you'd like to write code for us, great, welcome aboard! We have a few things
that you should be aware of.

## Developing Cirs

Use of feature gates is an important part of `cirs` library. This allows us to
provide the minimal implementation and then also provide platform specific
implementations for the target.

## Commit Style

For the matter of consistency, and because we use a changelog generator that
needs a particular style, we require the commit message to be in the following
format:

```
%type%(%scope%): %header%

%optional description%

%closes%

%optional signing%
```

Valid `type`s are:
- `feat`: Features
- `fix`: Bug fixes
- `docs`: Documentation
- `style`: Code style fixes
- `refactor`: Code refactors
- `perf`: Performance enhancements
- `test`: Tests
- `chore`: `TODO`s
- `revert`: revert previous commits

## Code Style

We follow Rust formatting and naming conventions. You can read more [here]. You
can install `rustfmt` and run `cargo fmt` to help with formatting. Some things
to note:

* Line endings
    * Use Unix line endings when committing (`\n`).
    * Windows users of git can do `git config --global core.autocrlf true` to
    let git convert them automatically.

* Column width
    * 100 characters
    * Feel free to wrap when it will help with readability

* Indentation
    * Use 4 spaces for indentations, do not use tabs.

* Deprecation
    * Be prepared to provide justification for the deprecation.
    * When deprecating, provide the month and year when it was deprecated.

* C enums, functions and structs
    * Since C standard structs have a defined standard name, they can/will
    break out of the styling rules set by rust. That is fine. Just add
    `#[allow(non_camel_case)]` on those items.

## Submitting your Pull Requests
In your PRs, please make sure you fulfill the following:

* If introducing new features for a `Gold` tier target, please add relevant
tests in the libcirs_tests directory. Thhis is a hard requirement for the PR to
be merged.
* Provide a justification for the change - is it a new feature or a fix to a
bug?
* Before sending a pull request ensure that your branch is up to date with the
branch you are targeting.
* Do not squash commits unless directed to do so, but please _rebase_ your
changes on top of master when you feel your changes are ready to be submitted -
**_Do not merge_**. We will squash the commits in a way we feel logical.

[here]: https://doc.rust-lang.org/1.0.0/style/README.html
