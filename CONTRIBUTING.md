# Contributing

## Branch policy

`main` is protected. Direct pushes are rejected.

| Rule | Value |
| --- | --- |
| Merges | Pull request only |
| Required status checks | `ci`, `security` |
| Required approvals | 1 (the owner may self-approve) |
| Force pushes | Blocked |
| Branch deletion | Blocked |
| Stale approvals | Dismissed on new commits |

Feature branches are short lived and are deleted after the pull request is merged.

### Branch naming

| Prefix | Use |
| --- | --- |
| `feat/<scope>-<slug>` | New user visible capability, e.g. `feat/api-echo-retry` |
| `fix/<scope>-<slug>` | Bug fix, e.g. `fix/web-echo-input` |
| `chore/<scope>-<slug>` | Maintenance, dependencies, tooling, e.g. `chore/ci-audit` |
| `docs/<scope>-<slug>` | Documentation only, e.g. `docs/branch-policy` |

`<scope>` is the package or area touched (`api`, `ui`, `web`, `desktop`, `mobile`, `ci`).

### Merging

Prefer squash merge so the history on `main` stays linear and one commit maps to one
change. Rebase merge is acceptable for long lived branches. The branch is deleted once the
pull request is merged.

## Commit messages

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) and are
validated by `commitlint` in CI.

```
<type>(<optional scope>): <subject>
```

Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`,
`chore`, `revert`.

Examples:

```
feat(api): add echo endpoint with uppercase option
fix(ui): clear the echo input after submit
ci: run the security audit on every pull request
```

## Local validation

Run the same checks CI runs before opening a pull request:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Commit messages can be checked locally with commitlint:

```sh
npm install
npm run lint:commit
```

## Pull requests

1. Branch off `main` following the naming rules above.
2. Keep the pull request focused; one logical change per pull request.
3. Make sure CI is green: `ci` (lint, test, build) and `security` (audit) must pass.
4. Request a review from the code owners listed in `.github/CODEOWNERS`.
5. Merge with squash and delete the branch.

## Releases

Releases are cut from `main` by pushing a semantic version tag:

```sh
git tag v0.1.0
git push origin v0.1.0
```

The `release` workflow then:

1. Builds the web bundle and the desktop application with `dx build --release`.
2. Packages them as `web-<tag>.tar.gz` and `desktop-<tag>-macos.zip`.
3. Publishes a container image for the web app to `ghcr.io`.
4. Attaches the artifacts to a GitHub release.

The `release` GitHub environment holds the deployment secrets and is restricted to version
tags. Mobile artifacts (apk/ipa) are optional and need the Android SDK/NDK, so they are
built on demand through the `workflow_dispatch` input; signing secrets are out of scope.

## Code owners

`packages/api` and `packages/ui` hold the shared logic used by every platform, so changes
there need a code owner review.
