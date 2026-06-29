package main

import (
	"runtime"
)

func os_detection() string {
	os := runtime.GOOS
	if os == "windows" {
		return "-Windows.exe"
	} else if os == "darwin" {
		return "-macOS"
	} else if os == "linux" {
		return "-Linux"
	} else {
		return "other"
	}
}

func arch_detection() string {
	arch := runtime.GOARCH
	if arch == "amd64" {
		return "-X64"
	} else if arch == "arm64" {
		return "-ARM64"
	} else {
		return "other"
	}
}
