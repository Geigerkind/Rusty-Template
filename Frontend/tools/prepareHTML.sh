cd "$(dirname "$0")";

mkdir ./../.html;

MASTER="./../index.master";
for dir in ./../html/*/
do
    HEAD=$(cat ${dir}head.html | sed -e 's/[\/&]/\\&/g');
    BODY=$(cat ${dir}body.html | sed -e 's/[\/&]/\\&/g');
    
    PATHTOFILE=$(echo "$dir" | cut -c 11-);
    NEWDIR="./../.html/$PATHTOFILE";
    NEWFILE="${NEWDIR}index.html";

    mkdir -p $NEWDIR;

    cp $MASTER $NEWFILE;

    sed -e "s/{head_placeholder}/$HEAD/g" $NEWFILE > $NEWFILE.tmp;
    sed -e "s/{body_placeholder}/$BODY/g" $NEWFILE.tmp > $NEWFILE;

    rm $NEWFILE.tmp;
done