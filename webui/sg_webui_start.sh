#!/bin/bash

# Check if directory exists
if [ ! -d "sg-view/node_modules" ]; then
    echo "Kill servers"
    killall node -q
    killall sg-history -q
fi
echo "VIEW PORT: $2"

# Check if port alive
check_view_return=$(lsof -i -P -n | grep LISTEN | grep ^node | grep :${2})
if [ -z "$check_view_return" ]; then

    # start a development version of the app
    echo "Launching View"
    cd sg-view
    node_modules/.bin/nuxt --hostname "0.0.0.0" &
    cd ../
fi
echo "Done with view"
echo "HISTORY PORT: ${3}"

# Check database
check_database_return=$(lsof -i -P -n | grep LISTEN | grep ^sg | grep :${3})
if [ -z "$check_database_return" ]; then

    # setup database
    echo "Launching Database"
    cd sg-history
    cargo run &
    cd ../
fi
echo "Done with database"
