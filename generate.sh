#!/usr/bin/env bash

set -e
set -x

port() {
	echo $((0x$(crc32 <(echo $1)) % 55536 + 10000))
}

if [[ -z $1 ]]; then
	echo "Usage: ./generate.sh <repo name>"
	exit 1
fi

echo Using name $1

git init
echo /generate.sh >> .git/info/exclude
git add .

git ls-files | xargs sed -i "s/rust-web-game-skeleton/$1/"

OLD_PORT=$(port rust-web-game-skeleton)
NEW_PORT=$(port $1)
git ls-files | xargs sed -i "s/$OLD_PORT/$NEW_PORT/"

git remote add origin git@github.com:SOF3/$1
git commit -am "Initial commit"
git push -u origin master

git checkout --orphan=gh-pages
git add README.md
git commit -am "Initial commit"
git push -u origin gh-pages

git checkout master

rm generate.sh
