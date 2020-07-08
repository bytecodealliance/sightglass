ORIG_CWD="$(pwd)"
SCRIPT_LOC="$(dirname ${BASH_SOURCE:-$0})"
cd ${SCRIPT_LOC}/build/wamr_app_aot/
echo $(git log -1 --pretty=format:"%at")
cd ${ORIG_CWD}