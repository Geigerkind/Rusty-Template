cd "$(dirname "$0")";

NUM_CORES=$1;

echo "Waiting for lessc to finish...";
while [ $(ps aux | grep lessc | wc -l) -gt 1 ]; do
    sleep 0.2;
done

echo "Merging files...";
# Manually creating the main files
for f in $(find ./../.tmp_css/main/ -name "*.css"); do
    PATHTOFILE=$(dirname "$f");
    cat $f >> "$PATHTOFILE/merge.css";
done

for f in $(find ./../.tmp_css/ -name "*.css" -not -path "./../.tmp_css/main/*"); do
    PATHTOFILE=$(dirname "$f");

    if [ ! -d "$PATHTOFILE" ]; then
        mkdir -p $PATHTOFILE;
    fi

    if [ ! -f "$PATHTOFILE/merge.css" ]; then
        cat "./../.tmp_css/main/merge.css" > "$PATHTOFILE/merge.css";
        case "$f" in
            *inline*)
                cat "./../.tmp_css/main/inline/merge.css" > "$PATHTOFILE/merge.css";
            ;;
        esac
    fi

    cat $f >> "$PATHTOFILE/merge.css";
done

echo "Minimizing CSS...";
for f in $(find ./../.tmp_css/ -name "merge.css"); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep csso | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 14-);
    NEWDIR="./../.css_merge/$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    csso --input $f --output $NEWDIR/style.css & 
done

echo "Waiting for csso to finish...";
while [ $(ps aux | grep csso | wc -l) -gt 1 ]; do
    sleep 0.2;
done

echo "Inlining CSS...";
mkdir "./../.html";
for f in $(find ./../html/ -name "*.html" -not -path "./../html/index.html"); do
    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 11-);
    NEWDIR="./../.html/$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    if [ ! -d "./../.css_merge/$PATHTOFILE" ] || [ ! -d "./../.css_merge/$PATHTOFILE/inline" ]; then
        continue;
    fi;
    
    INLINE_CSS=$(cat "./../.css_merge/$PATHTOFILE/inline/style.css" | sed -e 's/[\/&]/\\&/g');
    sed -e "s/{inline_css}/$INLINE_CSS/g" $f > $NEWDIR/$FILENAME.tmp;

    if [ -f "./../.css_merge/$PATHTOFILE/style.css" ]; then
        md5=($(md5sum "./../.css_merge/$PATHTOFILE/style.css"));
        mv "./../.css_merge/$PATHTOFILE/style.css" "./../.css_merge/$PATHTOFILE/style.css?$md5";
        FILE_PATH="/css/$PATHTOFILE/style.css?$md5";
        FILE_PATH=$(echo $FILE_PATH | sed -e 's/[\/&]/\\&/g');
        sed -e "s/{css}/$FILE_PATH/g" $NEWDIR/$FILENAME.tmp > $NEWDIR/$FILENAME;
    fi

    rm $NEWDIR/$FILENAME.tmp;
done

echo "Removing inline files...";
for f in $(find ./../.css_merge/ -name "*.css\?*" -not -path "*inline*"); do
    FILENAME=$(basename -- "$f");
    PATHTOFILE=$(dirname "$f" | cut -c 17-);
    NEWDIR="./../.css/$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    mv $f $NEWDIR/$FILENAME;
done

rm -r "./../.css_merge";
rm -r "./../.tmp_css";

