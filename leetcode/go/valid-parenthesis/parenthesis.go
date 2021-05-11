package parenthesis

import "strings"

func isValid(s string) bool {
	stack := make([]rune, 0)

	s = strings.TrimSpace(s)

	for _, c := range s {
		if len(stack) < 1 && (c == ')' || c == ']' || c == '}') {
			return false
		}

		switch c {
		case '{':
			stack = append(stack, c)
		case '[':
			stack = append(stack, c)
		case '(':
			stack = append(stack, c)
		case '}':
			e := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if e != '{' {
				return false
			}
		case ']':
			e := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if e != '[' {
				return false
			}
		case ')':
			e := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if e != '(' {
				return false
			}
		default:
			return false
		}
	}

	return len(stack) == 0
}

func IsValid(s string) bool {
	return isValid(s)
}
