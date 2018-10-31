package main

import (
	"io/ioutil"
	"fmt"
	"strings"
)

func main() {
	bytes, err := ioutil.ReadFile("input/Day05.txt")
	if err != nil {
		panic(err)
	}
	var input= string(bytes)
	var lines = strings.Split(input, "\n")

	part1 := 0
	part2 := 0
	for _, line := range lines {
		if isNice(line) && !isNaughty(line) {
			part1++
		}
		if isNicePart2(line) {
			part2++
		}
	}
	fmt.Println("Part 1", part1)
	fmt.Println("Part 2", part2)
}

func isNice(s string) bool {
	const vowels = "aeiou"
	vowelCount := 0
	for _, c := range s {

		if strings.Contains(vowels, string(c)) {
			vowelCount++
		}
	}
	if vowelCount < 3 {
		return false
	}

	for i := 1; i < len(s); i++ {
		if s[i] == s[i - 1] {
			return true
		}
	}
	return false
}


func isNaughty(s string) bool {
	if strings.Contains(s, "ab") {
		return true
	}
	if strings.Contains(s, "cd") {
		return true
	}
	if strings.Contains(s, "pq") {
		return true
	}
	if strings.Contains(s, "xy") {
		return true
	}
	return false
}

func isNicePart2(s string) bool {
	hasDoublePair := false
	for i := 0; i < len(s)-3; i++ {
		if strings.Contains(s[i + 2:], s[i:i + 2]) {
			hasDoublePair = true
			break
		}
	}

	hasDoubledChar := false
	for i := 0; i < len(s)-2; i++ {

		if s[i] == s[i + 2] {
			hasDoubledChar = true
			break
		}
	}

	return hasDoublePair && hasDoubledChar
}