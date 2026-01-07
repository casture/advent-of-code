package day3

import "fmt"

const (
	ROLLS_LIMIT = 4
)

type direction struct {
	dx, dy int
}

var directions []direction

func Run(lines []string) {
	directions = []direction{
		{-1, -1},
		{-1, 0},
		{-1, 1},
		{0, -1},
		{0, 1},
		{1, -1},
		{1, 0},
		{1, 1},
	}
	part1(lines, false)
	part2(lines)
}

func part1(lines []string, edit bool) int {
	var accessible int
	maxRow, maxCol := len(lines), len(lines[0])
	inBounds := func(i, j int) bool {
		return i >= 0 && j >= 0 && i < maxRow && j < maxCol
	}
	for i := range lines {
		var line string
		for j, c := range lines[i] {
			if c == '.' {
				line += string(c)
				continue
			}

			var rolls uint
			for _, d := range directions {
				if inBounds(i+d.dx, j+d.dy) && lines[i+d.dx][j+d.dy] == '@' {
					rolls += 1
				}
			}

			if rolls < ROLLS_LIMIT {
				accessible += 1
				line += "."
			} else {
				line += string(c)
			}
		}
		if edit {
			lines[i] = line
		}
		fmt.Println(line)
	}
	fmt.Printf("# forklift accessible: %d\n", accessible)
	return accessible
}

func part2(lines []string) {
	for _, l := range lines {
		fmt.Println(l)
	}
	var totalChanged int
	changed := 1
	for changed > 0 {
		changed = part1(lines, true)
		totalChanged += changed
	}
	fmt.Printf("total changed: %d\n", totalChanged)
}
