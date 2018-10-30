package main

import (
	"io/ioutil"
	"fmt"
)

func main() {
	Day03()
}

var santa_x, santa_y int
var houses = map[string] int {}
var robot_x, robot_y int

func Day03() {
	bytes, err := ioutil.ReadFile("input/Day03.txt")
	if err != nil {
		panic(err)
	}
	var input= string(bytes)
	//fmt.Println(input)
	houses["0,0"] = 1
	for _, c := range input {
		//fmt.Println(c)
		move(&santa_x, &santa_y, c)
	}
	fmt.Println(houses)
	fmt.Println("Part 1:", len(houses))

    // Part 2
    santa_x = 0
    santa_y = 0
	houses = map[string] int {}

	for i := 0; i < len(input); i += 2 {
		move(&santa_x, &santa_y, rune(input[i]))
		move(&robot_x, &robot_y, rune(input[i + 1]))
	}
	fmt.Println("Part 2:", len(houses))
}

func move(x *int, y *int, c rune) {
	switch c {
	case '<':
		*x--
	case '>':
		*x++
	case '^':
		*y++
	case 'v':
		*y--
	default:
		fmt.Println("Unknown char", c)
	}
	var address = fmt.Sprintf("%d,%d", *x, *y)
	houses[address] += 1
}