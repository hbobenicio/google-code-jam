package main

import (
	"bytes"
	"fmt"
)

func main() {
	var T int
	fmt.Scanf("%d", &T)

	for t := 1; t <= T; t++ {
		var n int
		fmt.Scanf("%d", &n)

		var lydia string
		fmt.Scanf("%s", &lydia)

		fmt.Printf("Case #%d: %s\n", t, solve(n, lydia))
	}
}

func solve(n int, lydia string) string {
	var b bytes.Buffer

	for _, c := range lydia {
		if c == 'S' {
			b.WriteRune('E')
		} else {
			b.WriteRune('S')
		}
	}

	return b.String()
}
