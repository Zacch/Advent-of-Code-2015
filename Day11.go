package main

import (
	"fmt"
	"strings"
)

const validLetters = "abcdefghjkmnpqrstuvwxyz"

func main() {
	part1 := nextPassword("hxbxwxba")
	fmt.Println("Part 1", part1)
	part2 := nextPassword(part1)
	fmt.Println("Part 2", part2)
}

func nextPassword(oldPassword string) string {
	nextPassword := oldPassword
	done := false
	for ;done == false; {
		nextPassword = increment(nextPassword)
		done = isValid(nextPassword)
	}
	return nextPassword
}

func increment(str string) string {
	incrementing := true
	reversedOutput := ""
	for i := len(str) - 1; i >= 0; i-- {
		if incrementing {
			if str[i] == 'z' {
				reversedOutput += string('a')
				continue
			}
			letterIndex := strings.Index(validLetters, string(str[i]))
			reversedOutput += string(validLetters[letterIndex + 1])
			incrementing = false
		} else {
			reversedOutput += string(str[i])
		}
	}
	return Reverse(reversedOutput)
}

func Reverse(s string) (result string) {
	for _,v := range s {
		result = string(v) + result
	}
	return result
}

func isValid(password string) bool {

	// Passwords must include one increasing straight of at least three letters,
	// like abc, bcd, cde, and so on.
	foundStraight := false
	for i := 2; i < len(password); i++ {
		if password[i-2]+1 == password[i-1] && password[i-1]+1 == password[i] {
			foundStraight = true
		}
	}
	if !foundStraight {
		return false
	}

	// Passwords may not contain the letters i, o, or l.
	if strings.ContainsAny(password, "iol") {
		return false
	}

	// Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
	pairs := 0
	var firstPairLetter byte = ' '

	for i := 0; i < len(password) - 1; i++ {
		if password[i] == firstPairLetter {
			continue
		}
		if password[i] == password[i + 1] {
			if pairs == 0 {
				firstPairLetter = password[i]
			}
			pairs++
		}
	}

	return pairs >= 2
}
