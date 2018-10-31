package main

import (
	"crypto/md5"
	"fmt"
	"io"
)


func main() {
	const secretKey = "bgvyzdsv"
	answer := -1
	hash := "foobar"

	for string(hash[0:5]) != "00000" {
		answer++
		h := md5.New()
		io.WriteString(h, fmt.Sprintf("%s%d", secretKey, answer))
		hash = fmt.Sprintf("%x", h.Sum(nil))
	}
	fmt.Println("Part 1:", answer)

	for string(hash[0:6]) != "000000" {
		answer++
		h := md5.New()
		io.WriteString(h, fmt.Sprintf("%s%d", secretKey, answer))
		hash = fmt.Sprintf("%x", h.Sum(nil))
	}
	fmt.Println("Part 2:", answer)
}
