package raindrops

import "strconv"
import "bytes"

func Convert(n int) string {
	i := 3

	var result bytes.Buffer
	for i <= n {
		if n%i == 0 {
			result.WriteString(translateText(i))
		}

		if i == n/2 {
			i = n
		} else {
			i++
		}
	}

	if result.Len() < 1 {
		result.WriteString(strconv.Itoa(n))
	}

	return result.String()
}

func translateText(n int) string {
	switch n {
	case 3:
		return "Pling"
	case 5:
		return "Plang"
	case 7:
		return "Plong"
	default:
		return ""
	}

}
