#!/usr/bin/env bash
set -euo pipefail

PROTO_REPO="https://github.com/CiphertextEcosystem/CiphertextEcosystemProtobuf.git"
PROTO_REF="main"
PROTO_DIR="proto"

if [ -f "$PROTO_DIR/model/ciphertext.proto" ]; then
  echo "Protos already present, skipping fetch."
  exit 0
fi

TMP_DIR="$(mktemp -d)"

echo "Fetching protobufs..."
git clone --depth=1 --branch "$PROTO_REF" "$PROTO_REPO" "$TMP_DIR"

mkdir -p "$PROTO_DIR"
cp -R "$TMP_DIR/model" "$PROTO_DIR/"

rm -rf "$TMP_DIR"
echo "Done."

