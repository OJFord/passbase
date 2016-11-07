#!/bin/sh
set -e

red="\033[31m"
green="\033[32m"
default="\033[39m"
outcome_pos="\033[74G"

failure() {
    printf $outcome_pos
    printf $red
    printf "FAILED\n"
    printf $default
    cat /tmp/test_err >> /tmp/test_failures
}

success() {
    printf $outcome_pos
    printf $green
    printf "OK\n"
    printf $default
}

should_pass() {
    $@ 1>/dev/null 2>/tmp/test_err || touch /tmp/test_failed
    printf '.'
}

should_fail() {
    $@ 1>/dev/null 2>/tmp/test_err && touch /tmp/test_failed
    printf '.'
}

describe() {
    printf "Testing $1.."
}

finish() {
    test -f /tmp/test_failed && failure || success
    rm -f /tmp/test_failed
}


# Setup
test_dir=$(dirname $(readlink -f "$0"))
export PATH="$test_dir/bin:$PATH"

keybase_dir="/keybase/private/$KEYBASE_USER"
sudo mkdir -p $keybase_dir
user=$(whoami)
sudo chown -R $user $keybase_dir

passbase_dir="$keybase_dir/.passbase"
config_file="$HOME/.passbase"

keybase_loc=$(which keybase)
hide_keybase() {
    sudo mv $keybase_loc /tmp/keybase
}
unhide_keybase() {
    sudo mv /tmp/keybase $keybase_loc
}
#

describe "list succeeds with no tags"
should_pass passbase list
finish

describe "failure with no config or Keybase"
rm $config_file
hide_keybase
should_fail passbase list
unhide_keybase
finish

describe "with .passbase config"
printf "{\"User\":\"$KEYBASE_USER\"}" > $config_file
hide_keybase
should_pass passbase list
unhide_keybase
finish

describe "update to .passbase config"
rm $config_file
should_pass passbase list
should_pass test -f $config_file
finish

describe "failure to access non-existent tag"
should_fail passbase read foo
should_fail passbase change foo
yes | should_fail passbase remove foo
finish

describe "creation of tag"
should_pass passbase create foo
should_pass test -f $passbase_dir/foo
should_pass passbase read foo
finish

describe "change to existing tag"
touch $passbase_dir/foo /tmp/foo_old
should_pass passbase change foo
should_fail cmp -s /tmp/foo_old $passbase_dir/foo
finish

describe "removal of existing tag"
touch $passbase_dir/foo
yes | should_pass passbase remove foo
should_fail test -f $passbase_dir/foo
finish

describe "'are you sure' prompt for removal"
touch $passbase_dir/bar
yes 'n' | should_pass passbase remove bar
should_pass test -f $passbase_dir/bar
finish

describe "'are you sure' removal prompt defaults to 'no'"
touch $passbase_dir/rab
yes '' | should_pass passbase remove rab
should_pass test -f $passbase_dir/rab
finish

describe "aliases"
should_pass passbase ls
should_pass passbase touch foobar
should_pass passbase cat foobar
yes | should_pass passbase rm foobar
finish

# Teardown
sudo rm -r /keybase/private/passbase_test
rm $config_file
cat /tmp/test_failures
#
