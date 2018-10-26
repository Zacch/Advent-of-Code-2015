package main

import (
	"fmt"
	"io/ioutil"
)

func main() {
	Day02()
}

func Day02() {
	bytes, err := ioutil.ReadFile("input/Day02.txt")
	if err != nil {
		panic(err)
	}
	var input = string(bytes)
	fmt.Print(input)
}
