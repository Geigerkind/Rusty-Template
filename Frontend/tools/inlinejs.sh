cd "$(dirname "$0")";

NUM_CORES=$1;

#if [ ! -d "./../.js" ]; then
#    mkdir "./../.js";
#    mkdir "./../.tmp_js";
#    mkdir "./../.js_merge";
#fi
#bash ./tsc.sh $NUM_CORES;

echo "Waiting for tsc to finish...";
while [ $(ps aux | grep tsc | wc -l) -gt 2 ]; do
    sleep 0.2;
done

echo "Minimizing js...";
for f in $(find ./../.tmp_js/ -name "merge.js"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep uglifyjs | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 13-);
    NEWDIR="./../.js_merge/$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    uglifyjs --compress --mangle --keep-fnames --warn $f --output $NEWDIR/script.js & 
done

echo "Waiting for uglifyjs to finish...";
while [ $(ps aux | grep uglifyjs | wc -l) -gt 1 ]; do
    sleep 0.2;
done

echo "Inlining JS...";
#mkdir "./../.html";
for f in $(find ./../html/ -name "*.html" -not -path "./../html/index.html"); do
    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 11-);
    NEWDIR="./../.html/$PATHTOFILE";
    
    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    if [ ! -d "./../.js_merge/$PATHTOFILE" ] || [ ! -d "./../.js_merge/$PATHTOFILE/inline" ]; then
        continue;
    fi;

    INLINE_CSS=$(cat "./../.js_merge/$PATHTOFILE/inline/script.js" | sed -e 's/[\/&]/\\&/g');
    sed -e "s/{inline_js}/$INLINE_CSS/g" $NEWDIR/$FILENAME > $NEWDIR/$FILENAME.tmp;

    if [ -f "./../.js_merge/$PATHTOFILE/script.js" ]; then
        md5=($(md5sum "./../.js_merge/$PATHTOFILE/script.js"));
        mv "./../.js_merge/$PATHTOFILE/script.js" "./../.js_merge/$PATHTOFILE/script.js?$md5";
        FILE_PATH="/js/$PATHTOFILE/script.js?$md5";
        FILE_PATH=$(echo $FILE_PATH | sed -e 's/[\/&]/\\&/g');
        sed -e "s/{js}/$FILE_PATH/g" $NEWDIR/$FILENAME.tmp > $NEWDIR/$FILENAME;
    fi

    rm $NEWDIR/$FILENAME.tmp;
done

echo "Removing inline files...";
for f in $(find ./../.js_merge/ -name "*.js\?*" -not -path "*inline*"); do
    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 16-);
    NEWDIR="./../.js/$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    mv $f $NEWDIR/$FILENAME;
done

rm -r "./../.js_merge";
rm -r "./../.tmp_js";

