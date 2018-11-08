package main

import (
	"io/ioutil"
	"strings"
	"fmt"
	"strconv"
)




func main() {
	bytes, err := ioutil.ReadFile("input/Day06.txt")
	if err != nil {
		panic(err)
	}
	var input= string(bytes)
	var lines = strings.Split(input, "\n")

	lights := [1000][1000]bool{}
	dimmerLights := [1000][1000]int{}

	for _, line := range lines {
		process(line, &lights, &dimmerLights)
	}

	fmt.Println("Part 1:", count(&lights))
	fmt.Println("Part 2:", sum(&dimmerLights))
}

func process(line string, lights *[1000][1000]bool, dimmerLights *[1000][1000]int) {
	var tokens = strings.Split(line, " ")
	switch tokens[1] {
	case "on":
		turnOn(tokens[2], tokens[4], lights)
		twistDimmers(1, tokens[2], tokens[4], dimmerLights)
	case "off":
		turnOff(tokens[2], tokens[4], lights)
		twistDimmers(-1, tokens[2], tokens[4], dimmerLights)
	default:
		toggle(tokens[1], tokens[3], lights)
		twistDimmers(2, tokens[1], tokens[3], dimmerLights)
	}
}

func count(lights *[1000][1000]bool) int {
	litLights := 0
	for y := 0; y < 1000; y++ {
		for x := 0; x < 1000; x++ {
			if lights[y][x] == true {
				litLights++
			}
		}
	}
	return litLights
}

func sum(dimmerLights *[1000][1000]int) interface{} {
	sum := 0
	for y := 0; y < 1000; y++ {
		for x := 0; x < 1000; x++ {
			sum += dimmerLights[y][x]
		}
	}
	return sum
}

type point struct {
	x, y int
}

func parse(s string) point {
	coordinates := strings.Split(s, ",")
	x, err := strconv.Atoi(coordinates[0])
	if err != nil {
		panic(fmt.Sprintf("Error converting x in %v: %v", s, err))
	}
	y, err := strconv.Atoi(coordinates[1])
	if err != nil {
		panic(fmt.Sprintf("Error converting y in %v: %v", s, err))
	}
	return point{x: x, y: y}
}

func turnOn(beginning string, end string, lights *[1000][1000]bool) {
	start := parse(beginning)
	stop := parse(end)
	for row := start.y; row <= stop.y; row++ {
		for column := start.x; column <= stop.x; column++ {
			lights[row][column] = true
		}
	}
}

func turnOff(beginning string, end string, lights *[1000][1000]bool) {
	start := parse(beginning)
	stop := parse(end)
	for row := start.y; row <= stop.y; row++ {
		for column := start.x; column <= stop.x; column++ {
			lights[row][column] = false
		}
	}
}

func toggle(beginning string, end string, lights *[1000][1000]bool) {
	start := parse(beginning)
	stop := parse(end)
	for row := start.y; row <= stop.y; row++ {
		for column := start.x; column <= stop.x; column++ {
			lights[row][column] = !lights[row][column]
		}
	}
}

func twistDimmers(amount int, beginning string, end string, dimmedLights *[1000][1000]int) {
	start := parse(beginning)
	stop := parse(end)
	for row := start.y; row <= stop.y; row++ {
		for column := start.x; column <= stop.x; column++ {
			dimmedLights[row][column] += amount
			if dimmedLights[row][column] < 0 {
				dimmedLights[row][column] = 0
			}
		}
	}
}
