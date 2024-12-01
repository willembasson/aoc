package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input, _ := os.ReadFile("input.txt")
	part1(string(input))
	part2(string(input))
}

func part1(input string) {
	var sum int
	var v1 []int
	var v2 []int
	lines := strings.Split(strings.TrimSpace(input), "\n")
	for _, line := range lines {
		parts := strings.Fields(line)
		left, _ := strconv.Atoi(parts[0])
		right, _ := strconv.Atoi(parts[1])
		v1 = append(v1, left)
		v2 = append(v2, right)
	}
	sort.Ints(v1)
	sort.Ints(v2)
	for i := 0; i < len(v1); i++ {
		if v1[i] > v2[i] {
			sum += v1[i] - v2[i]
		} else {
			sum += v2[i] - v1[i]
		}
	}
	fmt.Println(sum)
}

func part2(input string) {
	var sum int
	var v1 []int
	var v2 []int
	lines := strings.Split(strings.TrimSpace(input), "\n")
	for _, line := range lines {
		parts := strings.Fields(line)
		left, _ := strconv.Atoi(parts[0])
		right, _ := strconv.Atoi(parts[1])
		v1 = append(v1, left)
		v2 = append(v2, right)
	}
	hMap := make(map[int]int)
	for _, num := range v2 {
		hMap[num]++
	}
	for _, num := range v1 {
		sum += num * hMap[num]
	}

	fmt.Println(sum)
}
