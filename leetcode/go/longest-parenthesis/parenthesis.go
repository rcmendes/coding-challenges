package parenthesis

import (
	"fmt"
	"strings"
)

func longestValidParentheses(s string) int {
	pMaps := make([]bool, len(s))

	s = strings.TrimSpace(s)

	for i, c := range s {
		if c == ')' {
			lastOpenedIndex := lastOpenedIndexUnchecked(s, pMaps, i)
			if i > 0 && lastOpenedIndex >= 0 {
				pMaps[lastOpenedIndex] = true
				pMaps[i] = true
			}
		}
	}

	counter, maxCounter := 0, 0

	for _, v := range pMaps {
		if v {
			counter++
		}

		if maxCounter < counter {
			maxCounter = counter
		}

		if !v {
			counter = 0

		}
	}

	fmt.Printf("%q\n", s)
	fmt.Printf("%v\n", pMaps)
	fmt.Printf("count: %v\n\n", maxCounter)

	return maxCounter
}

func lastOpenedIndexUnchecked(source string, list []bool, currentIndex int) int {
	for i := currentIndex - 1; i >= 0; i-- {
		if !list[i] && source[i] == '(' {
			return i
		}
	}

	return -1
}
