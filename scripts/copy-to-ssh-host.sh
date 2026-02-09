#!/bin/bash


if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "ERROR on loading .env"
    exit 1
fi


if ! command -v sshpass &> /dev/null; then
    echo "ERROR 'sshpass' not installed. (Ubuntu: sudo apt install sshpass)"
    exit 1
fi

GIT_ROOT="$(git rev-parse --show-toplevel)"
cd "$GIT_ROOT" || exit 1


echo "--------------------------------------"
echo "Checking git status"

FILES=$(git ls-files --modified --others --exclude-standard --full-name )
#FILES=$(git ls-files --modified --others --exclude-standard --full-name && git diff --cached --name-only)

if [ -z "$FILES" ]; then
    echo "No uncommitted files"
    exit 0
fi

echo "--------------------------------------"
echo "This files where replaced on remote:"
echo "$FILES" | sed 's/^/  [DIR] -> /'
echo "--------------------------------------"

export SSHPASS=$SSH_PASS

echo "Start rsync"
echo "$FILES" | sshpass -e rsync -avR --files-from=- ./ "${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_DEST}"
echo "--------------------------------------"


if [ $? -eq 0 ]; then
  echo "Finish sync on $REMOTE_HOST"
else
    echo "ERROR"
fi