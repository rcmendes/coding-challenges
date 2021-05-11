package accumulate

type function func(string) string

func Accumulate(c []string, fn function) []string {
	result := make([]string, len(c))

	for i, v := range c {
		result[i] = fn(v)
	}
	return result
}
