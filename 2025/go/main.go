package main

import (
	"bufio"
	"log"
	"os"
	"strings"

	day1 "github.com/casture/aoc/01"
	day2 "github.com/casture/aoc/02"
	day3 "github.com/casture/aoc/03"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("Usage: go run main.go <day>")
	}
	file := os.Args[1]
	parts := strings.Split(file, "_")
	day := parts[0]
	l := lines("../input/" + file + ".txt")
	switch day {
	case "01":
		day1.Run(l)
	case "02":
		day2.Run(l)
	case "03":
		day3.Run(l)
	}
}

func lines(path string) []string {
	file, err := os.Open(path)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return lines
}
