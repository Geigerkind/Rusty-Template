cd "$(dirname "$0")";

NUM_CORES=$1;

for f in $(find ./../less/ -name "*.less"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep "lessc -ru=all" | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    FILENAME=$(basename -- "$f" | cut -d. -f1);
    PATHTOFILE=$(dirname "$f" | cut -c 10-);
    NEWDIR="./../.tmp_css$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    lessc -ru=all --insecure $f $NEWDIR/$FILENAME.css & 
done