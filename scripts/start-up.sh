JAVA_PATH=$(readlink -f $(which java) 2>/dev/null)


if [ -z "$JAVA_PATH" ]; then
    echo "ERROR: No java found"
    exit 1
fi


echo "--------------------------------------"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/../BeatBoxer-Engien/" || { echo "Dir not found"; exit 1; }
echo "--------------------------------------"
echo "Env load start gradlew"
echo "_________________________________________________________________________________________________"

./gradlew run
