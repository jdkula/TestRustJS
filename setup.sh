#!/usr/bin/env bash

MACHINE="$(uname -s)"
ARCH="$(uname -m)"

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
LOCAL_DIR="${SCRIPT_DIR}/.local"
TEMP_DIR="${SCRIPT_DIR}/.local/temp"
mkdir -p "${SCRIPT_DIR}/.local/temp"

if [[ "${MACHINE}" = "Darwin" ]]; then
  if [[ "${ARCH}" = "arm64" ]]; then
    curl -fsSL "https://github.com/protocolbuffers/protobuf/releases/download/v31.0/protoc-31.0-osx-aarch_64.zip" -o "${TEMP_DIR}/protoc.zip"
    if [[ $? -ne 0 ]]; then
      echo "Error occured when downloading protoc!"
    fi
    unzip "${TEMP_DIR}/protoc" -d "${LOCAL_DIR}"
    rm -rf "${TEMP_DIR}"
  else
    echo "Unknown architecture ${ARCH}"
    exit 1
  fi
else
  echo "Unknown OS ${MACHINE}"
  exit 1
fi