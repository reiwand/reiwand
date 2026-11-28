#!/usr/bin/env bash
# Keeps the GitHub Pages branch (gh-pages) of the repository as one commit, so the Flatpak objects
# of older releases do not pile up in its history.
#   scripts/pages-publish.sh fetch <dir>             current branch, or an empty one the first time
#   TOKEN=<token> scripts/pages-publish.sh push <dir> <message>
# GITHUB_REPOSITORY names the repository (owner/name); PAGES_REMOTE replaces its URL in tests.
set -euo pipefail
MODE=${1:?usage: $0 fetch|push <dir> [message]} DIR=${2:?}
REMOTE=${PAGES_REMOTE:-https://github.com/${GITHUB_REPOSITORY:?}}
BRANCH=gh-pages
auth() { printf 'AUTHORIZATION: basic %s' "$(printf 'x-access-token:%s' "${TOKEN:?}" | base64 -w0)"; }

case $MODE in
  fetch)
    if git ls-remote --exit-code --heads "$REMOTE" "$BRANCH" >/dev/null; then
      git clone --quiet --depth 1 --branch "$BRANCH" "$REMOTE" "$DIR"
    else
      git init --quiet -b "$BRANCH" "$DIR"
    fi
    ;;
  push)
    MESSAGE=${3:?a commit message is needed}
    cd "$DIR"
    git checkout --quiet --orphan "publish-$$"
    git add --all
    git -c user.name='github-actions[bot]' \
      -c user.email='41898282+github-actions[bot]@users.noreply.github.com' \
      commit --quiet --no-gpg-sign -m "$MESSAGE"
    git -c http.extraheader="$(auth)" push --quiet --force "$REMOTE" "HEAD:refs/heads/$BRANCH"
    ;;
  *)
    echo "unknown mode $MODE" >&2
    exit 2
    ;;
esac
