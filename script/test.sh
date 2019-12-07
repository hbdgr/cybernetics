#!/usr/bin/env bash

set -e
cd "$(dirname "$0")/../"

if [ -z ${DATABASE_PASSWORD+x} ]; then
	echo "required env DATABASE_PASSWORD is unset, exiting.."
	exit 1
fi

DATABASE_NAME="cybernetics_test"
export DATABASE_URL="postgres://cyber:${DATABASE_PASSWORD}@localhost/${DATABASE_NAME}"

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
