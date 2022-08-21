cd ..
grcov . --binary-path PATH_TO_YOUR_BINARIES_DIRECTORY -s . -t html --branch --ignore-not-existing -o ./docs/gcov-html
