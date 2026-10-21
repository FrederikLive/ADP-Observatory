#!/usr/bin/env bash
# Pinned, manually invoked experimental installer. Never pipes remote code to a shell.
set -euo pipefail
REPO="FrederikLive/ADP-Observatory"
SIGNER="$REPO/.github/workflows/prototype-packages.yml"
RUN_ID="" SHA="" DEST_DIR="$HOME/.local/bin"
usage() {
  echo "Usage: bash scripts/install-preview.sh --run-id ID --expected-sha FULL_SHA [--install-dir ABSOLUTE_PATH]"
  echo "Requires authenticated gh CLI. Installs a verified CI preview outside your ADP project."
}
fail() { printf 'ADP-Observatory installer: %s\n' "$*" >&2; exit 1; }
while (($#)); do
  case "$1" in
    --run-id|--expected-sha|--install-dir)
      (($# >= 2)) || fail "Missing value for $1"
      case "$1" in --run-id) RUN_ID="$2";; --expected-sha) SHA="$2";; --install-dir) DEST_DIR="$2";; esac
      shift 2 ;;
    --help|-h) usage; exit 0 ;;
    *) fail "Unrecognized argument: $1" ;;
  esac
done
[[ "$RUN_ID" =~ ^[0-9]+$ ]] || fail "Provide a numeric --run-id"
[[ "$SHA" =~ ^[a-fA-F0-9]{40}$ ]] || fail "Provide a full 40-character --expected-sha"
SHA="$(printf '%s' "$SHA" | tr 'A-F' 'a-f')"
[[ "$DEST_DIR" = /* ]] || fail "--install-dir must be absolute"
case "/$DEST_DIR/" in */../*|*/./*) fail "Path components . and .. are not allowed" ;; esac
command -v gh >/dev/null || fail "GitHub CLI (gh) must be installed and authenticated"
case "$(uname -s):$(uname -m)" in
  Linux:x86_64) PLATFORM=linux-x64 ;;
  Darwin:arm64) PLATFORM=macos-arm64 ;;
  *) fail "Unsupported system; only Linux x64 and macOS arm64 preview binaries are available" ;;
esac
get_field() { gh run view "$RUN_ID" -R "$REPO" --json "$1" --jq ".$1"; }
[[ "$(get_field conclusion)" == success ]] || fail "The selected CI run did not succeed"
[[ "$(get_field headSha)" == "$SHA" ]] || fail "The selected run does not match the expected commit SHA"
[[ "$(get_field workflowName)" == "Prototype binary packages" ]] || fail "Unexpected workflow"
[[ "$(get_field headBranch)" == main ]] || fail "Only main-branch builds are accepted"

TMP="$(mktemp -d "${TMPDIR:-/tmp}/adp-obs-install.XXXXXXXX")" || fail "Cannot make temporary directory"
STAGED=""
trap '[[ -z "$STAGED" ]] || rm -f -- "$STAGED"; rm -rf -- "$TMP"' EXIT
gh run download "$RUN_ID" -R "$REPO" -n "adp-observatory-$PLATFORM-prototype" -D "$TMP" \
  || fail "Cannot download CI artifact (it may have expired)"
BINARY="$TMP/adp-observatory"
[[ -f "$BINARY" && ! -L "$BINARY" && -f "$TMP/SHA256SUMS.txt" && ! -L "$TMP/SHA256SUMS.txt" ]] \
  || fail "Missing or unsafe artifact contents"
CHECKSUM="$(awk '$2=="adp-observatory" && $1 ~ /^[a-fA-F0-9]+$/ {print tolower($1)}' "$TMP/SHA256SUMS.txt")"
[[ "$CHECKSUM" =~ ^[a-f0-9]{64}$ ]] || fail "Missing/duplicate SHA-256 entry"
if command -v sha256sum >/dev/null; then
  ACTUAL="$(sha256sum "$BINARY" | cut -d' ' -f1)"
else
  ACTUAL="$(shasum -a 256 "$BINARY" | cut -d' ' -f1)"
fi
[[ "$CHECKSUM" == "$ACTUAL" ]] || fail "SHA-256 mismatch"
gh attestation verify "$BINARY" --repo "$REPO" --signer-workflow "$SIGNER" \
  --source-digest "$SHA" --source-ref refs/heads/main >/dev/null \
  || fail "Signature/provenance verification failed; refusing installation"

# Inspect the nearest existing parent BEFORE creating any new installation directories.
PROJECT="$(pwd -P)"
PARENT="$DEST_DIR"
while [[ ! -e "$PARENT" ]]; do PARENT="$(dirname "$PARENT")"; done
PARENT_REAL="$(cd "$PARENT" && pwd -P)"
case "$PARENT_REAL/" in "$PROJECT/"*) fail "Installation would create files inside the project" ;; esac
mkdir -p -- "$DEST_DIR" || fail "Cannot create installation directory"
[[ -d "$DEST_DIR" && ! -L "$DEST_DIR" ]] || fail "Installation directory is unsafe"
REAL="$(cd "$DEST_DIR" && pwd -P)"
case "$REAL/" in "$PROJECT/"*) fail "Installation must be outside the project" ;; esac
FINAL="$REAL/adp-observatory"
[[ ! -e "$FINAL" && ! -L "$FINAL" ]] || fail "Executable already exists; uninstall explicitly before replacing"
STAGED="$REAL/.adp-observatory-$$"
install -m 0755 "$BINARY" "$STAGED" || fail "Cannot stage verified binary"
[[ ! -e "$FINAL" && ! -L "$FINAL" ]] || fail "Destination appeared during installation"
mv -n -- "$STAGED" "$FINAL" || fail "Cannot install"
[[ -f "$FINAL" ]] || fail "Installation did not create the expected file"
STAGED=""
"$FINAL" --version || fail "Installed program failed its smoke check"
printf '\nInstalled verified preview: %s\nRun it from a project as: %s .\n' "$FINAL" "$FINAL"
printf 'Add %s to PATH manually to use: adp-observatory .\n' "$REAL"
printf 'To uninstall, delete: %s\n' "$FINAL"
