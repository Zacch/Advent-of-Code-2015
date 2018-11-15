package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main()  {
	bytes, err := ioutil.ReadFile("input/Day08.txt")
	if err != nil {
		panic(err)
	}
	var input = string(bytes)
	var lines = strings.Split(input, "\n")

	var charLength, representationLength int
	for _, line := range lines {
		representationLength += len(line)

		for i := 1; i < len(line) - 1; i++ {
			char := line[i]
			if char == '\\' {
				i++
				if line[i] == 'x' {
					i += 2
				}
			}
			charLength++
		}
	}
	fmt.Println("Part 1:", representationLength - charLength)

	var doubleEncodedLength int
	for _, line := range lines {
		doubleEncodedLength += 2
		for i := 0; i < len(line); i++ {
			char := line[i]
			if char == '\\' || char == '"' {
				doubleEncodedLength++
			}
			doubleEncodedLength++
		}
		fmt.Println(line, doubleEncodedLength)
	}
	fmt.Println("Part 2:", doubleEncodedLength - representationLength)
}
