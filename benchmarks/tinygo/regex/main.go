// TinyGo regex benchmark.
//
// This benchmark tests regular expression matching performance using
// the standard library regexp package.

package main

import (
	"fmt"
	"os"
	"regexp"
)

// WASM imports for sightglass API
//
//go:wasm-module bench
//export start
func benchStart()

//go:wasm-module bench
//export end
func benchEnd()

func main() {
	// Read the input text file
	path := "tinygo-regex.input"
	fmt.Fprintf(os.Stderr, "[regex] matching %s\n", path)
	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
		os.Exit(1)
	}
	text := string(data)

	// Use the same patterns as the Rust benchmark
	emailPattern := `[\w\.+-]+@[\w\.-]+\.[\w\.-]+`
	uriPattern := `[\w]+://[^/\s?#]+[^\s?#]+(?:\?[^\s#]*)?(?:#[^\s]*)?`
	ipPattern := `(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9])`

	benchStart()

	emails := countMatches(text, emailPattern)
	uris := countMatches(text, uriPattern)
	ips := countMatches(text, ipPattern)

	benchEnd()

	fmt.Fprintf(os.Stderr, "[regex] found %d emails\n", emails)
	fmt.Fprintf(os.Stderr, "[regex] found %d URIs\n", uris)
	fmt.Fprintf(os.Stderr, "[regex] found %d IPs\n", ips)
}

func countMatches(text, pattern string) int {
	re := regexp.MustCompile(pattern)
	return len(re.FindAllString(text, -1))
}
