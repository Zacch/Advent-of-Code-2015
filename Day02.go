package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
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
	var lines = strings.Split(input, "\n")

	part1 := 0
	part2 := 0
	for i, line := range lines {
		var dimensionStr = strings.Split(line, "x")
		var dimensions = mapToSortedInts(dimensionStr)

		// Paper size
		size := 3 * dimensions[0] * dimensions[1]
		size += 2 * dimensions[0] * dimensions[2]
		size += 2 * dimensions[1] * dimensions[2]
		part1 += size

		// Ribbon
		volume := dimensions[0] * dimensions[1] * dimensions[2]
		ribbon := 2*(dimensions[0]+dimensions[1]) + volume
		part2 += ribbon
		fmt.Println(i, dimensions, size, ribbon)
	}
	fmt.Println("Part 1", part1)
	fmt.Println("Part 2", part2)
}

func mapToSortedInts(strs []string) []int {
	result := make([]int, len(strs))
	for i, s := range strs {
		var err error
		result[i], err = strconv.Atoi(s)
		if err != nil {
			panic(err)
		}
	}
	sort.Ints(result)
	return result
}
