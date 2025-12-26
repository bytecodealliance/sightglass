function main(input) {
  // Compile various regex patterns
  const patterns = [
    /\b[A-Z][a-z]+\b/g, // Capitalized words
    /\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b/g, // IP addresses
    /[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/g, // Email addresses
    /https?:\/\/[^\s]+/g, // URLs
    /\b[A-Z]{2,}\b/g, // Acronyms
    /\b\w{10,}\b/g, // Long words (10+ chars)
    /\d{4}-\d{2}-\d{2}/g, // ISO dates
    /\$\d+\.?\d*/g, // Dollar amounts
  ];

  let totalMatches = 0;
  for (const pattern of patterns) {
    const matches = input.match(pattern);
    if (matches) {
      totalMatches += matches.length;
    }
  }

  return `[spidermonkey-regex] compiled ${patterns.length} patterns\n[spidermonkey-regex] found ${totalMatches} total matches\n[spidermonkey-regex] input size: ${input.length} bytes\n`;
}
