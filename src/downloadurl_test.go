package main

import (
	"runtime"
	"testing"
)

func TestURL(t *testing.T) {
	actual := get_download()

	if runtime.GOOS == "windows" {
		if runtime.GOARCH == "amd64" {
			expect := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Windows.exe"
			if actual != expect {
				t.Error("Expected: ", expect, "Got: ", actual)
			}
		} else if runtime.GOARCH == "arm64" {
			expect := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Windows.exe"
			if actual != expect {
				t.Error("Expected: ", expect, "Got: ", actual)
			}
		}
	} else if runtime.GOOS == "darwin" {
		if runtime.GOARCH == "arm64" {
			expect := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-macOS"
			if actual != expect {
				t.Error("Expected: ", expect, "Got: ", actual)
			}
		} else if runtime.GOARCH == "amd64" {
			expect := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-macOS"
			if actual != expect {
				t.Error("Expected: ", expect, "Got: ", actual)
			}
		}
	} else if runtime.GOOS == "linux" {
		if runtime.GOARCH == "arm64" {
			expect := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-ARM64-Linux"
			if actual != expect {
				t.Error("Expected: ", expect, "Got: ", actual)
			}
		} else if runtime.GOARCH == "amd64" {
			expect := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Linux"
			if actual != expect {
				t.Error("Expected: ", expect, "Got: ", actual)
			}
		}
	}
}
