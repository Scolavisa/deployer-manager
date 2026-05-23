#!/bin/bash
# Bump the version across all config files.
# Usage: ./scripts/bump-version.sh <new-version>
# Example: ./scripts/bump-version.sh 1.2.0

set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <new-version>"
  echo "Example: $0 1.2.0"
  exit 1
fi

NEW_VERSION="$1"

# Validate semver format
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
  echo "Error: Version must be in semver format (e.g., 1.2.3)"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "Bumping version to $NEW_VERSION..."

# 1. package.json
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" "$PROJECT_DIR/package.json"
echo "  ✓ package.json"

# 2. src-tauri/Cargo.toml (only the package version, not dependency versions)
sed -i "0,/^version = \"[^\"]*\"/s/^version = \"[^\"]*\"/version = \"$NEW_VERSION\"/" "$PROJECT_DIR/src-tauri/Cargo.toml"
echo "  ✓ src-tauri/Cargo.toml"

# 3. src-tauri/tauri.conf.json
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" "$PROJECT_DIR/src-tauri/tauri.conf.json"
echo "  ✓ src-tauri/tauri.conf.json"

echo ""
echo "Version bumped to $NEW_VERSION in all files."
echo ""
echo "Next steps:"
echo "  1. git add -A"
echo "  2. git commit -m \"chore: bump version to $NEW_VERSION\""
echo "  3. git tag v$NEW_VERSION"
echo "  4. git push && git push --tags"
