// Package gigasecond provides a function that adds a gigasecond to an specific time.
package gigasecond

import (
	"time"
)

// Gigasecond represents a single gigasecond.
const gigasecond = time.Second * 1e9

// AddGigasecond takes a time, t and adds a gigasecond to it.
func AddGigasecond(t time.Time) time.Time {
	return t.Add(gigasecond)
}
