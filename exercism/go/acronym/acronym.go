// Package acronym converts a phrase into an acronym.
package acronym

import (
	"regexp"
	"strings"
)

// Abbreviate converts a phrase into an acronym.
func Abbreviate(s string) string {
	upper := strings.ToUpper(s)

	reg := regexp.MustCompile("[^\\s,-]+")
	words := reg.FindAllString(upper, -1)

	acronym := ""
	for _, w := range words {
		acronym += string(w[0])
	}
	return acronym
}
