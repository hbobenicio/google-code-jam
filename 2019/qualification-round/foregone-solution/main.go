// GCJ 2019 - Qualification Round - Foregone Solution
// hbobenicio@gmail.com
package main

import (
	"bytes"
	"fmt"
)

func main() {
	var T int
	var nstr string

	fmt.Scanf("%d", &T)

	for t := 1; t <= T; t++ {
		fmt.Scanf("%s", &nstr)

		a, b := solve(nstr)
		fmt.Printf("Case #%d: %s %s\n", t, a, b)
	}
}

func solve(s string) (string, string) {
	var a, b bytes.Buffer

	foundFour := false
	for _, c := range s {
		if c == '4' {
			a.WriteRune('2')
			b.WriteRune('2')
			foundFour = true
		} else {
			if !foundFour {
				a.WriteRune(c)
			} else {
				a.WriteRune(c)
				b.WriteRune('0')
			}
		}
	}

	return a.String(), b.String()
}
