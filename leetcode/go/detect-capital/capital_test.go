package capital

import "testing"

func TestCapital(t *testing.T) {
	testCases := []testCase{
		testCase{input: "USA", output: true},
		testCase{input: "FlaG", output: false},
		testCase{input: "leetcode", output: true},
		testCase{input: "Google", output: true},
	}

	for _, test := range testCases {
		result := detectCapitalUse(test.input)
		if test.output != result {
			t.Fatalf("Failed on evaluate '%s'. Expected '%v' but got '%v'", test.input, test.output, result)
		}
	}
}
