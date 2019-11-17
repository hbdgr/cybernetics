#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/../"

DATABASE_NAME="cybernetics_test"
export DATABASE_URL="postgres://cyber:cyber@localhost/${DATABASE_NAME}"

arg=$1
case $arg in
	"setup")
		diesel setup
		diesel migration run
		;;
	"run")
		export ROCKET_ADDRESS=localhost
		export ROCKET_PORT=8000

		diesel database reset
		cargo test
		;;
	*)
		printf "Bad option.\n Available options: setup, run\n";
		;;
esac
shift
