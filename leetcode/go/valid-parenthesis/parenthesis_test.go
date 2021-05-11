package parenthesis

import "testing"

type inputResult struct {
	value  string
	result bool
}

//TestValid Tests isValid function
func TestValid(t *testing.T) {
	inputs := []inputResult{
		inputResult{value: "", result: true},
		inputResult{value: "abc", result: false},
		inputResult{value: "()", result: true},
		inputResult{value: "()[]{}", result: true},
		inputResult{value: "(]", result: false},
		inputResult{value: "{[]}", result: true},
		inputResult{value: "{[}]", result: false},
		inputResult{value: "([)]", result: false},
		inputResult{value: "(", result: false},
		inputResult{value: "[", result: false},
		inputResult{value: "{", result: false},
		inputResult{value: ")", result: false},
		inputResult{value: "]", result: false},
		inputResult{value: "}", result: false},
	}

	for _, input := range inputs {
		valid := IsValid(input.value)
		if valid != input.result {
			t.Fatalf("Failed %v. Expected %v, but got %v", input.value, input.result, valid)
		}
	}
}
