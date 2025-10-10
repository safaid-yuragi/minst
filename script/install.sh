#!/bin/sh

set -e

VERSION="v0.1.1"
BASE_URL="https://github.com/sirasaki-konoha/minst/releases/download/${VERSION}"

detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux" ;;
        Darwin*)    echo "macos" ;;
        MINGW*|MSYS*|CYGWIN*) echo "windows" ;;
        *)          echo "unknown" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64" ;;
        aarch64|arm64)  echo "aarch64" ;;
        *)              echo "unknown" ;;
    esac
}

OS=$(detect_os)
ARCH=$(detect_arch)

if [ "$OS" = "unknown" ] || [ "$ARCH" = "unknown" ]; then
    echo "Error: Unsupported OS/architecture"
    echo "OS: $OS, ARCH: $ARCH"
    exit 1
fi

if [ "$OS" = "windows" ]; then
	FILENAME="minst-${OS}-${ARCH}.exe"
	DOWNLOAD_FILE_LOCATION="$(mktemp).exe"
else
	FILENAME="minst-${OS}-${ARCH}"
	DOWNLOAD_FILE_LOCATION="$(mktemp)"
fi

DOWNLOAD_URL="${BASE_URL}/${FILENAME}"

echo "Detected system: ${OS} ${ARCH}"
echo "Downloading installer..."

if command -v curl >/dev/null 2>&1; then
    curl -sSfL -o "$DOWNLOAD_FILE_LOCATION" "$DOWNLOAD_URL" 
elif command -v wget >/dev/null 2>&1; then
    wget -q -O "$DOWNLOAD_FILE_LOCATION" "$DOWNLOAD_URL"
else
    echo "Error: curl or wget is required"
    exit 1
fi

if [ "$OS" != "windows" ]; then
    chmod +x "$DOWNLOAD_FILE_LOCATION"
fi

echo ""

"$DOWNLOAD_FILE_LOCATION" "$@"
rm -f "$DOWNLOAD_FILE_LOCATION"
