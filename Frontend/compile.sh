cd "$(dirname "$0")";

BUILD_MODE=$1;
BUILD_TYPE=0; # Debug
if [ "$BUILD_MODE" == "release" ]; then
    BUILD_TYPE=1;
fi
FORCE_ASSET_REPROCESSING=$2;
NUM_CORES=$(grep -c ^processor /proc/cpuinfo);
MEDIA_COMPILE_DIR=".media";
TARGET_DIR="/var/www";
APP_NAME="jaylapp";

# Optimizing the images
LAST_MEDIA_CHANGE=$(find ./media -printf "%T@\n" | sort | tail -1);
SAVED_MEDIA_CHANGE="";
if [ -f ".media_change" ]; then
    SAVED_MEDIA_CHANGE=$(cat .media_change);
fi

if [ "$SAVED_MEDIA_CHANGE" == "$LAST_MEDIA_CHANGE" ] && [ -d "$MEDIA_COMPILE_DIR" ] && [ "$FORCE_ASSET_REPROCESSING" != "true" ]; then
    echo "Images did not change, using saved...";
else
    echo "Optimizing images...";
    if [ ! -d "$MEDIA_COMPILE_DIR" ]; then
        mkdir $MEDIA_COMPILE_DIR;
    fi

    bash ./tools/zopfli.sh $NUM_CORES "./../$MEDIA_COMPILE_DIR";
    bash ./tools/guetzli.sh $NUM_CORES "./../$MEDIA_COMPILE_DIR";
    bash ./tools/webpc.sh $NUM_CORES "./../$MEDIA_COMPILE_DIR";

    echo $LAST_MEDIA_CHANGE > .media_change;
fi

# Compiling TS to JS
echo "Compiling TypeScript...";
if [ ! -d ".js" ]; then
    mkdir ".js";
    mkdir ".tmp_js";
    mkdir ".js_merge";
fi
bash ./tools/tsc.sh $NUM_CORES;

echo "Merging/Minimizing/Inlining JS...";
bash ./tools/inlinejs.sh $NUM_CORES;

# Compiling Less to CSS
echo "Compiling LessCSS...";
if [ ! -d ".css" ]; then
    mkdir ".css";
    mkdir ".tmp_css";
    mkdir ".css_merge";
fi
bash ./tools/lessc.sh $NUM_CORES;

echo "Merging/Minimizing/Inlining CSS...";
bash ./tools/inlinecss.sh $NUM_CORES;

# Minimize resulting HTML
echo "Minimizing HTML...";
bash ./tools/htmlmin.sh $NUM_CORES;

echo "Waiting for jobs to finish...";
while [ $(ps aux | grep htmlminhelper | wc -l) -gt 1 ]; do
    sleep 0.2;
done

mv .html/main/index.html .html/index.html;
rm -r .html/main;

echo "Building directory...";
rm -r $TARGET_DIR/$APP_NAME;
mkdir -p $TARGET_DIR/$APP_NAME;

if [ $BUILD_TYPE -eq 0 ]; then
    cp -r ./media $TARGET_DIR/$APP_NAME/;
else
    cp -r ./.media $TARGET_DIR/$APP_NAME/;
fi
mv ./.css $TARGET_DIR/$APP_NAME/css;
mv ./.js $TARGET_DIR/$APP_NAME/js;
mv ./.html/* $TARGET_DIR/$APP_NAME/;
rm -r ./.html;

# Web manifest
cp ./manifest.json $TARGET_DIR/$APP_NAME/;

# Serviceworker
cp ./serviceworker.js $TARGET_DIR/$APP_NAME/;

# Robots
cp ./robots.txt $TARGET_DIR/$APP_NAME/;

echo "Build has been moved to $TARGET_DIR/$APP_NAME!";
