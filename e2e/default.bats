# bats test_tags=default
@test "empty body" {
    run bash -c 'echo "feat(cli): impl -a flag" | commitlint'
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "body max length" {
    run bash -c "echo \"feat(cli): impl -a flag
    
Hello, I'm longer than 72 charactures. I'm very long so that it is not suitable as it's hard to read many long commit messages.
Make it smart. But we should not be opinionated so the default is ignored.\" | commitlint"
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "description empty" {
    run bash -c 'echo "feat(cli): " | commitlint'
    [ "$status" -eq 1 ]
}

# bats test_tags=default
@test "description format" {
    run bash -c 'echo "feat(other): add script" | commitlint'
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "scope" {
    run bash -c 'echo "feat(other): add script" | commitlint'
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "scope format" {
    run bash -c 'echo "feat(other): add script" | commitlint'
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "subject empty" {
    run bash -c 'echo "feat(other): add script" | commitlint'
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "type" {
    run bash -c 'echo "feat(other): add script" | commitlint'
    [ "$status" -eq 0 ]
}

# bats test_tags=default
@test "type empty" {
    run bash -c 'echo "(other): add script" | commitlint'
    [ "$status" -eq 1 ]
}

# bats test_tags=default
@test "type format" {
    run bash -c 'echo "feat(other): add script" | commitlint'
    [ "$status" -eq 0 ]
}
