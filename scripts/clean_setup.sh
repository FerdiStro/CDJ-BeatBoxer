echo "Kill all java instances"
sudo killall -9 java

echo "Check pods:"
sudo ss -lupn | grep -E '50000|50001|50002|60000|60001'

echo "Needs to be empty when not use 'sudo kill -9 <PID>'"