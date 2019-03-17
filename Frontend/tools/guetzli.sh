cd "$(dirname "$0")";
echo "JPG ...";

NUM_CORES=$1;
MEDIA_DIR=$2;

for f in $(find ./../media/ -name "*.jpg"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep "guetzli --quality" | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 11-);
    NEWDIR="$MEDIA_DIR$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    guetzli --quality 84 --nomemlimit $f $NEWDIR/$FILENAME > /dev/null 2> /dev/null & 
done