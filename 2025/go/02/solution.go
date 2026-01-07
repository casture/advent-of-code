package day1

import (
	"fmt"
	"strconv"
	"strings"
)

func Run(lines []string) {
	part1(lines)
	part2(lines)
}

func part1(lines []string) {
	for i, line := range lines {
		sum, ranges := 0, strings.Split(line, ",")
		for _, ran := range ranges {
			spl := strings.Split(ran, "-")
			lS, rS := spl[0], spl[1]

			l, _ := strconv.Atoi(lS)
			r, _ := strconv.Atoi(rS)
			for ; l <= r; l++ {
				if lS[:len(lS)/2] == lS[len(lS)/2:] {
					sum += l
				}
			}
		}
		fmt.Println(i, sum)
	}
}

func part2(lines []string) {
	for i, line := range lines {
		sum, ranges := 0, strings.Split(line, ",")
		for _, ran := range ranges {
			spl := strings.Split(ran, "-")
			lS, rS := spl[0], spl[1]

			l, _ := strconv.Atoi(lS)
			r, _ := strconv.Atoi(rS)
			for ; l <= r; l++ {
				if invalid(l) {
					// fmtPrintln("invalid: ", l)
					sum += l
				}
			}
		}
		fmt.Println(i, sum)
	}
}

func invalid(v int) bool {
	s := strconv.Itoa(v)
	stop := len(s) / 2
	for i := range stop {
		ix := i + 1
		sub := s[:ix]
		check := true
		for j := range (len(s) / ix) - 1 {
			x := (j + 1) * ix
			fmt.Println(sub, s[x:x+ix], x, x+ix)
			if sub != s[x:x+ix] {
				check = false
				break
			}
		}
		if check {
			return true
		}
	}
	return false
}
