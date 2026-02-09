#!/bin/sh

set -e

APP_ID="dev.heppen.webapps"

gh_repo="papirus-icon-theme"
gh_desc="Papirus icon theme"

: "${XDG_DATA_HOME:=$HOME/.local/share}"
: "${TAG:=master}"

EXTRA_THEMES="Papirus Papirus-Dark Papirus-Light"

temp_file="$(mktemp)"
temp_dir="$(mktemp -d)"

cleanup() {
    echo "Clearing cache ..."
    rm -rf "$temp_file" "$temp_dir"
    echo "Done!"
}

trap cleanup EXIT HUP INT TERM

download() {
    echo "Getting the latest version from GitHub ..."
    wget -O "$temp_file" \
        "https://github.com/PapirusDevelopmentTeam/$gh_repo/archive/$TAG.tar.gz"
    echo "Unpacking archive ..."
    tar -xzf "$temp_file" -C "$temp_dir"
}

install() {
    dest="$1"
    shift

    for theme in "$@"; do
        test -d "$temp_dir/$gh_repo-$TAG/$theme" || continue
        echo "Installing '$theme' ..."
        cp -R "$temp_dir/$gh_repo-$TAG/$theme" "$dest"
    done
}

download

install_path="$XDG_DATA_HOME/$APP_ID/icons"

mkdir -p "$install_path"

install "$install_path" $EXTRA_THEMES
