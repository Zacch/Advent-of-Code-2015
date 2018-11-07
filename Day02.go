package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
	"sync"
)

func main() {
	Day02()
}

var (
	mutex        sync.Mutex
	waitGroup    sync.WaitGroup
	part1, part2 int
)

const numberOfThreads = 6

func Day02() {
	bytes, err := ioutil.ReadFile("input/Day02.txt")
	if err != nil {
		panic(err)
	}
	var input = string(bytes)
	var lines = strings.Split(input, "\r\n")

	lineChannel := make(chan string, numberOfThreads)
	for n := 0; n < numberOfThreads; n++ {
		waitGroup.Add(1)
		go calculate(lineChannel)
	}

	for _, line := range lines {
		lineChannel <- line
	}
	close(lineChannel)

	waitGroup.Wait()

	fmt.Println("Part 1", part1)
	fmt.Println("Part 2", part2)
}

func calculate(lineChannel chan string) {
	for line := range lineChannel {
		var dimensionStr = strings.Split(line, "x")
		var dimensions = mapToSortedInts(dimensionStr)
		// Paper size
		paperSize := 3 * dimensions[0] * dimensions[1]
		paperSize += 2 * dimensions[0] * dimensions[2]
		paperSize += 2 * dimensions[1] * dimensions[2]
		// Ribbon
		volume := dimensions[0] * dimensions[1] * dimensions[2]
		ribbonLength := 2*(dimensions[0]+dimensions[1]) + volume

		// fmt.Println(line, dimensions, paperSize, ribbonLength)
		mutex.Lock()
		part1 += paperSize
		part2 += ribbonLength
		mutex.Unlock()
	}
	waitGroup.Done()
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
