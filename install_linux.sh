#!/bin/bash

set -e

detect_package_manager() {
    if command -v dnf &> /dev/null; then
        echo "dnf"
    elif command -v yum &> /dev/null; then
        echo "yum"
    elif command -v apt-get &> /dev/null; then
        echo "apt"
    elif command -v pacman &> /dev/null; then
        echo "pacman"
    elif command -v zypper &> /dev/null; then
        echo "zypper"
    else
        echo "unknown"
    fi
}

install_with_dnf() {
    sudo dnf install -y mpv yt-dlp ffmpeg socat
}

install_with_yum() {
    sudo yum install -y epel-release
    sudo yum install -y mpv ffmpeg socat
    install_ytdlp_pip
}

install_with_apt() {
    sudo apt-get update
    sudo apt-get install -y mpv yt-dlp ffmpeg socat
}

install_with_pacman() {
    sudo pacman -Sy --noconfirm mpv yt-dlp ffmpeg socat
}

install_with_zypper() {
    sudo zypper install -y mpv yt-dlp ffmpeg socat
}

install_ytdlp_pip() {
    if ! command -v yt-dlp &> /dev/null; then
        if command -v pip3 &> /dev/null; then
            pip3 install --user yt-dlp
        elif command -v pip &> /dev/null; then
            pip install --user yt-dlp
        else
            curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o ~/.local/bin/yt-dlp
            chmod a+rx ~/.local/bin/yt-dlp
        fi
    fi
}

check_dependencies() {
    local missing=()

    if ! command -v mpv &> /dev/null; then
        missing+=("mpv")
    fi

    if ! command -v yt-dlp &> /dev/null; then
        missing+=("yt-dlp")
    fi

    if ! command -v ffprobe &> /dev/null; then
        missing+=("ffmpeg")
    fi

    if ! command -v socat &> /dev/null; then
        missing+=("socat")
    fi

    if [ ${#missing[@]} -eq 0 ]; then
        return 0
    else
        echo "missing: ${missing[*]}"
        return 1
    fi
}

main() {
    echo "gs-music dependency installer"

    if check_dependencies; then
        echo "you already have all the dependencies installed!"
        exit 0
    fi

    package_manager=$(detect_package_manager)
    echo "detected package manager: $package_manager"

    case $package_manager in
        dnf)
            install_with_dnf
            ;;
        yum)
            install_with_yum
            ;;
        apt)
            install_with_apt
            ;;
        pacman)
            install_with_pacman
            ;;
        zypper)
            install_with_zypper
            ;;
        *)
            echo "error: unsupported package manager"
            echo "please manually install: mpv yt-dlp ffmpeg socat"
            exit 1
            ;;
    esac

    echo ""
    echo "verifying installation..."
    if check_dependencies; then
        echo ""
        echo "install complete! you can now download gs-music"
    else
        echo "warning: some dependencies may not have installed correctly"
        exit 1
    fi
}

main "$@"
