echo "------------------------------------"
echo "Build Engine"
echo "------------------------------------"

cd BeatBoxer-Engien/
./gradlew clean
./gradlew bundleApp


echo "------------------------------------"
echo "Build TUI"
echo "------------------------------------"
cd ../BeatBoxer-Tui/
cargo build --release --bin prod_binary

cd ..
mv BeatBoxer-Tui/target/release/prod_binary CDJ-BeatBoxer