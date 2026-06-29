package main

import (
	"runtime"
)

func url_test() {
	get_download()
	if runtime.GOOS == "windows" {
		if runtime.GOARCH == "amd64" {
			if get_download() == "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin-X64-Windows.exe" {
				x := true
			}
		}
	}
}
