// Resolve `WASMTIME_REF` (a branch, a tag, or a full commit hash; `main` by
// default) to a full commit hash and emit it as the step's `sha` output. Used
// by the `callgrind` workflow; see `callgrind.yml`.

import { strict as assert } from "node:assert";
import { execFileSync } from "node:child_process";
import * as fs from "node:fs";

const WASMTIME_REPOSITORY = "https://github.com/bytecodealliance/wasmtime/";

async function main() {
  const ref = process.env.WASMTIME_REF || "main";
  let sha;
  if (/^[0-9a-f]{40}$/.test(ref)) {
    sha = ref;
  } else {
    const refs = execFileSync("git", ["ls-remote", WASMTIME_REPOSITORY, ref], {
      encoding: "utf8",
    });
    sha = refs.split("\n")[0]?.split("\t")[0];
  }
  assert.match(
    sha ?? "",
    /^[0-9a-f]{40}$/,
    `unable to resolve wasmtime ref \`${ref}\``,
  );
  fs.appendFileSync(process.env.GITHUB_OUTPUT, `sha=${sha}\n`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
