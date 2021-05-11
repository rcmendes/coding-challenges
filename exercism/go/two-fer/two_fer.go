/*
Package twofer or `2-fer` is short for two for one. One for you and one for me.

```text
"One for X, one for me."
```

When X is a name or "you".

If the given name is "Alice", the result should be "One for Alice, one for me."
If no name is given, the result should be "One for you, one for me."
*/
package twofer

import (
	"fmt"
)

// ShareWith should have a comment documenting it.
func ShareWith(n string) string {
	name := n
	if name == "" {
		name = "you"
	}

	return fmt.Sprintf("One for %s, one for me.", name)
}
