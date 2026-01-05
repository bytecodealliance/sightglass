// TinyGo JSON serialization and deserialization benchmark.
//
// This benchmark tests JSON parsing and serialization performance using
// the standard library encoding/json package.

package main

import (
	"encoding/json"
	"fmt"
	"os"
)

// WASM imports for sightglass API
//
//go:wasm-module bench
//export start
func benchStart()

//go:wasm-module bench
//export end
func benchEnd()

// Data structures matching the Rust benchmark
type User struct {
	ID        uint64   `json:"id"`
	Username  string   `json:"username"`
	Email     string   `json:"email"`
	FullName  string   `json:"full_name"`
	IsActive  bool     `json:"is_active"`
	CreatedAt string   `json:"created_at"`
	UpdatedAt string   `json:"updated_at"`
	Profile   Profile  `json:"profile"`
	Settings  Settings `json:"settings"`
	Posts     []Post   `json:"posts"`
}

type Profile struct {
	Bio         string   `json:"bio"`
	AvatarURL   string   `json:"avatar_url"`
	Location    string   `json:"location"`
	Website     *string  `json:"website"`
	SocialLinks []string `json:"social_links"`
}

type Settings struct {
	Theme                string            `json:"theme"`
	Language             string            `json:"language"`
	NotificationsEnabled bool              `json:"notifications_enabled"`
	PrivacyLevel         string            `json:"privacy_level"`
	Preferences          map[string]string `json:"preferences"`
}

type Post struct {
	ID            uint64   `json:"id"`
	Title         string   `json:"title"`
	Content       string   `json:"content"`
	Tags          []string `json:"tags"`
	Likes         uint32   `json:"likes"`
	CommentsCount uint32   `json:"comments_count"`
	PublishedAt   string   `json:"published_at"`
}

func main() {
	// Read the JSON input file
	jsonData, err := os.ReadFile("tinygo-json.input")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
		os.Exit(1)
	}

	benchStart()

	// Deserialize: Parse JSON into structured data
	var users []User
	if err := json.Unmarshal(jsonData, &users); err != nil {
		fmt.Fprintf(os.Stderr, "Error parsing JSON: %v\n", err)
		os.Exit(1)
	}

	// Serialize: Convert back to JSON
	serialized, err := json.Marshal(users)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error serializing JSON: %v\n", err)
		os.Exit(1)
	}

	benchEnd()

	fmt.Fprintf(os.Stderr, "[tinygo-json] processed %d users\n", len(users))
	fmt.Fprintf(os.Stderr, "[tinygo-json] serialized size: %d bytes\n", len(serialized))
}
