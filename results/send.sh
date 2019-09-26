#!/bin/bash

if [ "$#" -ne 7 ]; then
    echo ""
    echo "Send data in json format to sg history UI"
    echo ""
    echo "Usage"
    echo "-----"
    echo "./send.sh <history_submit_url> <unique_submission_id> <abi-source> <info_message> <vm> <int_time_stamp> <json_datafile>"
    echo ""
    echo "Example:"
    echo "      ./submit_data.sh localhost:8001/submit 123456 x86-test_server \"gitref: cb387\" lucet_app 1566422278 results-latest.json"  
    echo ""
    echo "Note: time_stamp currently "
    echo ""
    exit 1
fi

echo "curl ${1} -X POST -F revision_id=${2} -F author=${3} -F message=\"${4}\" -F gitref=${5} -F ts=${6} -F results=${7}" 
curl ${1} -X POST -F "revision_id=${2}" -F "author=${3}" -F "message=\"${4}\"" -F "gitref=${5}" -F "ts=${6}" -F "results=$(cat ${7})"

exit 0