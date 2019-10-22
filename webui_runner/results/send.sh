#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo ""
    echo "Send data in json format to sg history UI"
    echo ""
    echo "Usage"
    echo "-----"
    echo "./send.sh <history_submit_url> <json_datafile>"
    echo ""
    echo "Example:"
    echo "      ./submit_data.sh localhost:8001/submit 123456 results-latest.json"
    echo ""
    echo "Note: time_stamp currently "
    echo ""
    exit 1
fi

echo "curl -vX POST ${1} -d @${2} --header \"Content-Type: application/json\""
curl -v ${1} -d @${2} --header "Content-Type: application/json"
exit 0