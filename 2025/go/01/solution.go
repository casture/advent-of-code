package day2

import (
	"fmt"
	"log"
	"strconv"
)

const (
	START = 50
	MAX   = 100
)

func Run(lines []string) {
	part1(lines)
	part2(lines)
}

func part1(lines []string) {
	var sum int
	cur := START
	for _, line := range lines {
		if line == "" {
			continue
		}
		dir := 1
		if line[0] == 'L' {
			dir = -1
		}
		step, err := strconv.Atoi(line[1:])
		if err != nil {
			log.Fatal(err)
		}

		for _ = range step {
			cur = (cur + dir + MAX) % MAX
		}

		if cur == 0 {
			sum++
		}
	}
	fmt.Println("Part 1: ", sum)
}

func part2(lines []string) {
	var sum int
	cur := START
	for _, line := range lines {
		if line == "" {
			continue
		}
		dir := 1
		if line[0] == 'L' {
			dir = -1
		}
		step, err := strconv.Atoi(line[1:])
		if err != nil {
			log.Fatal(err)
		}

		for _ = range step {
			cur = (cur + dir + MAX) % MAX

			if cur == 0 {
				sum++
			}
		}
	}
	fmt.Println("Part 2: ", sum)
}
