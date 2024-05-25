#!/usr/bin/expect -f

set cmd [lindex $argv 0]

# Spawn the pick command with some options
spawn bash -c "echo 'option 1\noption 2\noption 3' | $cmd"

# Send the selection (e.g., the first option)
send -- "1\r"

expect "option 1"

# Expect the end
expect eof

# Spawn the pick command with some options and a custom delimiter
spawn bash -c "echo 'option 1,option 2,option 3' | $cmd -d ','"

# Send the selection (e.g., the first option)
send -- "2\r"

expect "option 2"

# Expect the end
expect eof