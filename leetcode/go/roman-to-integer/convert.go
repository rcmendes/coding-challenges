package convert

import "strings"

func romanToInt(s string) int {
	s = strings.TrimSpace(s)
	if len(s) < 1 {
		return 0
	}

	characters := []rune(s)
	sum := 0
	lastDistinctValue := 0
	for i:= len(characters) -1; i >= 0; i-- {
		value := convertRomanLetterToInt(characters[i])
		if value >= lastDistinctValue || lastDistinctValue == 0 {
			sum += value
		} else {
			sum -=value
		}
		
		lastDistinctValue = value
	}

	return sum
}

func convertRomanLetterToInt(letter rune) int {
	switch letter {
	case 'I':
		return 1
	case 'V':
		return 5
	case 'X':
		return 10
	case 'L':
		return 50
	case 'C':
		return 100
	case 'D':
		return 500
	case 'M':
		return 1000
	default:
		return 0
	}
}
