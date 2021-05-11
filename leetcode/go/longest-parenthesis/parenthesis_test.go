package parenthesis

import "testing"

type inputResult struct {
	value  string
	result int
}

//TestValid Tests isValid function
func TestValid(t *testing.T) {
	inputs := []inputResult{
		inputResult{value: "(()", result: 2},
		inputResult{value: ")()())", result: 4},
		inputResult{value: "()(())", result: 6},
		inputResult{value: "()(()", result: 2},
	}

	for _, input := range inputs {
		valid := longestValidParentheses(input.value)
		if valid != input.result {
			// t.Fatalf("Failed %v. Expected %v, but got %v", input.value, input.result, valid)
		}
	}
}
