cd "$(dirname "$0")";

NUM_CORES=$1;

GLOBAL_FILES="";
for f in $(find ./../ts/global/ -name "*.ts" -not -path "./../ts/global/inline/*"| sort); do
    if [ "$GLOBAL_FILES" == "" ]; then
        GLOBAL_FILES=$f;
    else
        GLOBAL_FILES="$GLOBAL_FILES $f";
    fi
done

GLOBAL_INLINE_FILES="";
for f in $(find ./../ts/global/inline/ -name "*.ts" | sort); do
    if [$GLOBAL_INLINE_FILES == ""]; then
        GLOBAL_INLINE_FILES=$f;
    else
        GLOBAL_INLINE_FILES="$GLOBAL_INLINE_FILES $f";
    fi
done

FILES="";
LAST_PATHTOFILE="";
for f in $(find ./../ts/ -name "*.ts" -not -path "./../ts/global/*" | sort); do
    # Only start a process if < NUM_CORES are running
    while [ $(ps aux | grep "tsc --alwaysStrict" | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    PATHTOFILE=$(dirname "$f" | cut -c 8-);
    NEWDIR="./../.tmp_js$PATHTOFILE";

    if [ ! -d "$NEWDIR" ]; then
        mkdir -p $NEWDIR;
    fi

    if [ "$PATHTOFILE" != "$LAST_PATHTOFILE" ] && [ "$LAST_PATHTOFILE" != "" ] && [ "$FILES" != "" ]; then
        PREPEND_FILES=$GLOBAL_FILES;
        case "$LAST_PATHTOFILE" in
            *inline*)
                PREPEND_FILES=$GLOBAL_INLINE_FILES;
            ;;
        esac

        echo "Compiling $PREPEND_FILES $FILES...";
        tsc --alwaysStrict --baseUrl "/" --target "es6" --locale "en" --newLine "lf" --strict $PREPEND_FILES $FILES --outFile ./../.tmp_js$LAST_PATHTOFILE/merge.js & 
        FILES=$f;
        LAST_PATHTOFILE=$PATHTOFILE;
    else
        FILES="$FILES $f";
        LAST_PATHTOFILE=$PATHTOFILE;
    fi
done
if [ "$FILES" != "" ]; then
    while [ $(ps aux | grep "tsc --alwaysStrict" | wc -l) -gt $NUM_CORES ]; do
        sleep 0.5;
    done

    PREPEND_FILES=$GLOBAL_FILES;
    case "$LAST_PATHTOFILE" in
        *inline*)
            PREPEND_FILES=$GLOBAL_INLINE_FILES;
        ;;
    esac

    echo "Compiling $PREPEND_FILES $FILES...";
    tsc --alwaysStrict --baseUrl "/" --target "es6" --locale "en" --newLine "lf" --strict $PREPEND_FILES $FILES --outFile ./../.tmp_js$LAST_PATHTOFILE/merge.js & 
fi

while [ $(ps aux | grep "tsc --alwaysStrict" | wc -l) -gt 1 ]; do
    sleep 0.5;
done