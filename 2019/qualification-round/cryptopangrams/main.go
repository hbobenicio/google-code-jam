package main

import (
	"bytes"
	"fmt"
	"math/big"
	"sort"
)

func main() {
	var T int
	fmt.Scanf("%d", &T)

	for t := 1; t <= T; t++ {
		var l int
		var n string
		var x int64
		var ciphertext []*big.Int

		fmt.Scanf("%s %d", &n, &l)

		for i := 0; i < l; i++ {
			fmt.Scanf("%d", &x)
			ciphertext = append(ciphertext, big.NewInt(x))
		}

		fmt.Printf("Case #%d: %s\n", t, solve(ciphertext))
	}
}

func solve(ciphertext []*big.Int) string {
	var primes []*big.Int

	for i := 1; i < len(ciphertext); i++ {
		x, y := ciphertext[i-1], ciphertext[i]

		p := big.NewInt(1)
		p.GCD(nil, nil, x, y)
		primes = append(primes, p)
	}

	nfirst, nlast := ciphertext[0], ciphertext[len(ciphertext)-1]
	pfirst, plast := primes[0], primes[len(primes)-1]
	p1, pn := big.NewInt(1), big.NewInt(1)
	p1.Div(nfirst, pfirst)
	pn.Div(nlast, plast)

	primes = append([]*big.Int{p1}, primes...)
	primes = append(primes, pn)

	alreadyPresent := make(map[int64]bool)
	var primesMapping []*big.Int
	for _, p := range primes {
		if _, exists := alreadyPresent[p.Int64()]; !exists {
			alreadyPresent[p.Int64()] = true
			primesMapping = append(primesMapping, p)
		}
	}

	// go <= 1.7 ... this is sad =(
	sort.Sort(sortInt(primesMapping))

	// sort.Slice(primesMapping, func(i, j int) bool {
	// 	return primesMapping[i].Cmp(primesMapping[j]) < 0
	// })

	m := make(map[int64]rune)
	for i, p := range primesMapping {
		m[p.Int64()] = rune(i + 65)
	}

	var b bytes.Buffer
	for _, x := range primes {
		r := m[x.Int64()]
		b.WriteRune(r)
	}

	return b.String()
}

// Google Code Jam 2019 Judge uses old go version (1.7)
// To sort it, you need this sad stuff below...

type sortInt []*big.Int

func (si sortInt) Len() int {
	return len(si)
}

func (si sortInt) Swap(i, j int) {
	si[i], si[j] = si[j], si[i]
}

func (si sortInt) Less(i, j int) bool {
	return si[i].Cmp(si[j]) < 0
}
