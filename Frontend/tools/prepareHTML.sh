cd "$(dirname "$0")";

mkdir ./../.html;

MASTER="./../index.master";
for dir in ./../html/*/
do
    PATHTOFILE=$(echo "$dir" | cut -c 11-);
    NEWDIR="./../.html/$PATHTOFILE";
    NEWFILE="${NEWDIR}index.html";

    mkdir -p $NEWDIR;

    cp ${dir}head.html ${NEWDIR}index.head;
    cp ${dir}body.html ${NEWDIR}index.body;

    bash ./htmlminhelper.sh ${NEWDIR}index.head 1;
    bash ./htmlminhelper.sh ${NEWDIR}index.body 1;

    HEAD=$(cat ${NEWDIR}index.head | sed -e 's/[\/&]/\\&/g');
    BODY=$(cat ${NEWDIR}index.body | sed -e 's/[\/&]/\\&/g');

    rm -f ${NEWDIR}index.head;
    rm -f ${NEWDIR}index.body;

    cp $MASTER $NEWFILE;

    sed -e "s/{head_placeholder}/$HEAD/g" $NEWFILE > $NEWFILE.tmp;
    sed -e "s/{body_placeholder}/$BODY/g" $NEWFILE.tmp > $NEWFILE;

    rm $NEWFILE.tmp;
done