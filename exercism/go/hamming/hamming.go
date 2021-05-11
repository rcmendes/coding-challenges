package hamming

import "errors"

func Distance(a, b string) (int, error) {
	distance := 0

	if len(a) != len(b) {
		return -1, errors.New("Strands have different sizes")
	}

	for i := range a {
		if a[i] != b[i] {
			distance++
		}
	}

	return distance, nil

}
