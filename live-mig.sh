rm ./checkpoint/*
PID=$(pidof test.out)
criu dump -D ./checkpoint -t $PID -j -v4
criu restore -D ./checkpoint -j