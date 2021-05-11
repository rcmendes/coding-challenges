package main

import "fmt"

func main() {
	fmt.Printf("sum: %d, result: %v\n", 12, find(12))
	fmt.Printf("sum: %d, result: %v\n", 1000, find(1000))
}

type Triple struct {
	a int
	b int
	c int
}

type Pair struct {
	a int
	b int
}

type HashSet = []Triple

func find(sum int) HashSet {
	result := make(HashSet, 0)

	for i := 1; i <= sum/3; i++ {
		for j := i + 1; j <= sum/2; j++ {
			k := sum - i - j
			if i*i+j*j == k*k {
				result = append(result, Triple{i, j, k})
			}
		}
	}

	return result
}
