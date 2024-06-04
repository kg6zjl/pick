#!/usr/bin/expect -f

# we pass in the binary to run integration tests against
set cmd [lindex $argv 0]

############ TEST 1 ############
# tests default delimiter/default behavior
# Spawn the pick command with default options
spawn bash -c "echo 'option 1\noption 2\noption 3' | $cmd"

# Send the selection (e.g., the first option)
send -- "1\r"

expect "option 1"

# Expect the end
expect eof

############ TEST 2 ############
# tests a custom delimiter
# Spawn the pick command with some options and a custom delimiter
spawn bash -c "echo 'option 1,option 2,option 3' | $cmd -d ','"

# Send the selection (e.g., the second option)
send -- "2\r"

expect "option 2"

# Expect the end
expect eof

############ TEST 3 ############
# test the version output
# Spawn the pick command with --version flag
spawn bash -c "$cmd -V"

# expect output to say Pick x.x.x
expect -re "Pick (\[0-9]+.\[0-9]+.\[0-9]+)"

# Expect the end
expect eof

############ TEST 4 ############
# tests for empty piped data and no args
# Spawn the pick command with no options
spawn bash -c "$cmd"

# expect output to say Pick x.x.x
expect "Please provide arguments or piped data. Exiting."

# Expect the end
expect eof

############ TEST 5 ############
# tests for empty piped data and no args
# Spawn the pick command with some options and a custom delimiter
spawn bash -c "$cmd -d ','"

# expect output to say Pick x.x.x
expect "No input received. Exiting."


############ TEST 6 ############
# tests for column use case
exp_internal 1
spawn bash -c "echo ' M tests/integration_test.sh\n?? foo' | $cmd -c 2"

# Send the selection (e.g., the first option)
send -- "1\r"

expect "tests/integration_test.sh"

# Expect the end
expect eof
