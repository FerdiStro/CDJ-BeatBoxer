echo "--------------------------------------"
echo "Run copy-to-ssh-host.sh script for sync projects"
./copy-to-ssh-host.sh
echo "--------------------------------------"

if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "ERROR on loading .env"
    exit 1
fi


echo "Run application on ${REMOTE_USER}@${REMOTE_HOST}"

sshpass -p "${SSH_PASS}" ssh -tt "${REMOTE_USER}@${REMOTE_HOST}" "bash -i -c 'cd ${REMOTE_DEST}/scripts/ && ./start-up.sh'"