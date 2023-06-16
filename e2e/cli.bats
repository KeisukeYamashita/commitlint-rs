# bats test_tags=cli
@test "not existing config file" {
    run bash -c 'echo "feat(cli): impl -a flag" | commitlint --config not-existing-config.js'
    [ "$status" -eq 1 ]
}
