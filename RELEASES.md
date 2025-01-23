# Releases

This is a concise guide for cutting a release for `maili`.

This also applies to `op-alloy`, but _NOT_ to kona since kona uses [release-plz][release-plz].

### cargo-release

Ensure [cargo-release][cargo-release] is installed using cargo's `install` command.

```
$ cargo install cargo-release
```

### Dry Run

> [!TIP]
>
> Ensure that you have trunk (the `main` branch) checked out and up to date.

Execute the following command to perform a _dry run_ release.

```
$ cargo release --no-push patch
```

This will update the _patch_ version of crates in unison. (e.g. `0.1.0` -> `0.1.1`).

To update minor and major versions, just specify `minor` or `major` in place of `patch`.

If this command executes without any errors, proceed to executing the release.

### Cutting the Release

> [!IMPORTANT]
>
> Executing the release command may take a couple or a few minutes depending on your machine
> and how quickly it can compile all the crates. Be prepared to leave this running for some time.

Append the `--execute` argument to the cargo release command to execute the dry run above.

```
$ cargo release --no-push patch --execute
```

Crates will be published iteratively.
Once this is done be sure to push the artifacts in the next step!

### Upstreaming Artifacts

After the release command completes, it will automatically commit some artifacts to `main`.

We don't want to push to the `main` branch so we need to do a few things.

Reset the git commit so changes are not committed like so.

```
$ git reset HEAD^
```

Running `git status` should show unstaged changes to the `CHANGELOG.md` among other artifacts.

Now, checkout a new branch, commit, and push the artifacts to the new branch.

```
$ git checkout -b release/0.1.1
$ git add .
$ git commit -m "release: 0.1.1"
$ git push
```

Open a PR and you're all set, the release is complete!


<!-- Hyperlinks -->

[cargo-release]: https://github.com/crate-ci/cargo-release
[release-plz]: https://github.com/release-plz/release-plz
