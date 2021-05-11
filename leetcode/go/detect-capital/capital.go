package capital

import "strings"

type testCase struct {
	input  string
	output bool
}

// We define the usage of capitals in a word to be right when one of the following cases holds:

// All letters in this word are capitals, like "USA".
// All letters in this word are not capitals, like "leetcode".
// Only the first letter in this word is capital if it has more than one letter, like "Google".
func detectCapitalUse(word string) bool {
	wordLowerCase := strings.ToLower(word)
	wordUpperCase := strings.ToUpper(word)
	wordTitleCase := strings.Title(wordLowerCase)

	return (word == wordLowerCase) || (word == wordUpperCase) || (word == wordTitleCase)
}
