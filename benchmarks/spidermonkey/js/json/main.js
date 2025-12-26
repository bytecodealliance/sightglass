function main(input) {
  // Parse the JSON input
  const users = JSON.parse(input);

  // Serialize it back to JSON
  const serialized = JSON.stringify(users);

  return `[spidermonkey-json] processed ${users.length} users\n[spidermonkey-json] serialized size: ${serialized.length} bytes\n`;
}
