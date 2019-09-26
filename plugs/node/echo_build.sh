ORIG_CWD="$(pwd)"
SCRIPT_LOC="$(dirname ${BASH_SOURCE:-$0})"
cd ${SCRIPT_LOC}/build/
echo $(node -v)
cd ${ORIG_CWD}