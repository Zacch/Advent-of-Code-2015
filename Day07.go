package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type instruction struct {
	op string
	in1 string
	in2 string
	out string
}
var instructions []instruction

var wires = make(map[string]int)

var startingWires []string

func main() {
	bytes, err := ioutil.ReadFile("input/Day07.txt")
	if err != nil {
		panic(err)
	}
	var input = string(bytes)
	var lines = strings.Split(input, "\n")

	for _, line := range lines {
		parseLine(line)
	}
	execute()

	part1 := wires["a"]
	fmt.Println("Part 1", part1)

	//-------
	wires =  make(map[string]int)

	for _, wire := range startingWires {
		wires[wire] = 0
	}
	wires["b"] = part1
	execute()

	part2 := wires["a"]
	fmt.Println("Part 2", part2)
}

func parseLine(line string) {
	tokens := strings.Split(line, " ")
	if len(tokens) < 3 {
		return
	}
	if tokens[0] == "NOT" {
		instructions = append(instructions, instruction{
			op:  tokens[0],
			in1: tokens[1],
			in2: "",
			out: tokens[3],
		})
	} else {
		if tokens[1] == "->" {
			instr := instruction{
				op:  tokens[1],
				in1: tokens[0],
				in2: "",
				out: tokens[2],
			}
			input, err := strconv.Atoi(instr.in1)
			if err == nil {
				wires[instr.out] = input
				startingWires = append(startingWires, instr.out)
			} else {
				instructions = append(instructions, instr)
			}
		} else {
			instructions = append(instructions, instruction{
				op:  tokens[1],
				in1: tokens[0],
				in2: tokens[2],
				out: tokens[4],
			})
		}
	}
}

func execute() {
	for done := false; !done; {
		valueAdded := false
		for _, instr := range instructions {

			if inputsKnown(instr) && !outputKnown(instr) {
				calculateWire(instr)
				valueAdded = true
			}
		}
		if !valueAdded {
			done = true
		}
	}
}

func inputsKnown(i instruction) bool {
	_, in1Known := wires[i.in1]
	in1Known = in1Known || isNumeric(i.in1)

	if i.op == "NOT" || i.op == "->" {
		return in1Known
	}

	_, in2Known := wires[i.in2]
	in2Known = in2Known || isNumeric(i.in2)

	return in1Known && in2Known
}

func isNumeric(s string) bool {
	_, err := strconv.Atoi(s)
	return err == nil
}

func outputKnown(i instruction) bool {
	_, outputKnown := wires[i.out]
	return outputKnown
}

func calculateWire(instr instruction) {
	in1, err := strconv.Atoi(instr.in1)
	if err != nil {
		in1 = wires[instr.in1]
	}
	in2, err2 := strconv.Atoi(instr.in2)
	if err2 != nil {
		in2 = wires[instr.in2]
	}
	switch instr.op {
	case "->":
		wires[instr.out] = in1
	case "NOT":
		wires[instr.out] = in1 ^ 0xffff
	case "AND":
		wires[instr.out] = in1 & in2
	case "OR":
		wires[instr.out] = in1 | in2
	case "LSHIFT":
		wires[instr.out] = in1 << uint(in2)
	case "RSHIFT":
		wires[instr.out] = in1 >> uint(in2)
	default:
		panic(fmt.Errorf("unknown opcode: %s", instr.op))
	}
}