//Package bob implements a chat with bob.
package bob

import (
	"regexp"
	"strings"
)

// Hey function answers a question.
func Hey(remark string) string {
	trimmedMark := strings.TrimSpace(remark)
	length := len(trimmedMark)
	upperCase := strings.ToUpper(trimmedMark)
	hasLetter, _ := regexp.MatchString("[a-zA-Z]", trimmedMark)

	//He says 'Fine. Be that way!' if you address him without actually saying anything.
	if length == 0 {
		return "Fine. Be that way!"
	}

	//He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
	if p := trimmedMark[length-1]; p == '?' && upperCase == trimmedMark && hasLetter {
		return "Calm down, I know what I'm doing!"
	}

	// He answers 'Whoa, chill out!' if you yell at him.
	if upperCase == trimmedMark && hasLetter {
		return "Whoa, chill out!"
	}

	//Bob answers 'Sure.' if you ask him a question.
	if p := trimmedMark[length-1]; p == '?' {
		return "Sure."
	}

	//He answers 'Whatever.' to anything else.
	return "Whatever."
}
