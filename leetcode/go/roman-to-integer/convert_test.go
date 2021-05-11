package convert

import "testing"

type testCase struct {
	Input  string
	Output int
}

var testCases = []testCase{
	testCase{Input: "III",
		Output: 3},
	testCase{
		Input:  "IV",
		Output: 4},
	testCase{
		Input:  "IX",
		Output: 9},
	testCase{
		Input:  "LVIII",
		Output: 58},
	testCase{
		Input:  "MCMXCIV",
		Output: 1994},
}

func TestRomanToInteger(t *testing.T) {
	for _, test := range testCases {
		result := romanToInt(test.Input)
		if result != test.Output {
			t.Fatalf("Failed on converting %v to integer number. Expected %v, but got %v.", test.Input, test.Output, result)
		}
	}
}
