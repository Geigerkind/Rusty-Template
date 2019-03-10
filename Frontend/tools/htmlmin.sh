cd "$(dirname "$0")";

NUM_CORES=$1;

for f in $(find ./../.html/ -name "*index.html"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep htmlminhelper | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    bash ./htmlminhelper.sh $f &
done