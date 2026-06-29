package main

import "fmt"

func get_download() string {
	base_url := "https://github.com/Pumpkin-MC/Pumpkin/releases/download/nightly/pumpkin"
	donwload := base_url + arch_detection() + os_detection()
	fmt.Println(donwload)
	return donwload
}
