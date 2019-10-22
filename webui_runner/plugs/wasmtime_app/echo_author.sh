ORIG_CWD="$(pwd)"
SCRIPT_LOC="$(dirname ${BASH_SOURCE:-$0})"
cd ${SCRIPT_LOC}/build/wasmtime/
echo $(git log -1 --pretty=format:"%aN")
cd ${ORIG_CWD}