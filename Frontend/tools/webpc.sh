cd "$(dirname "$0")";
echo "WEBP ...";

NUM_CORES=$1;
MEDIA_DIR=$2;

for f in $(find ./../media/ -name "*.png" -o -name "*.jpg"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep "webpc -q=" | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    FILENAME=$(basename -- "$f" | cut -d. -f1);
    PATHTOFILE=$(dirname "$f" | cut -c 11-);
    NEWDIR="$MEDIA_DIR$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    webpc -q=30 $f $NEWDIR/$FILENAME.webp > /dev/null 2> /dev/null & 
done
