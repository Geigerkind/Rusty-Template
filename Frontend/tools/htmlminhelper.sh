# Before minifying replace all kinds of tags
if [ "$2" != "1" ]; then
    DESCRIPTION="Learn japanese in a structured manner from ground up. You will progress little by little by using a unique leveling system.";
    THEME_COLOR="#fff"
    sed -e "s/{meta_description}/$DESCRIPTION/g" $1 > $1.tmp;
    sed -e "s/{meta_theme_color}/$THEME_COLOR/g" $1.tmp > $1;
fi
#rm $1;
#mv $1.tmp $1;
html-minifier --collapse-whitespace --remove-empty-tags --remove-comments --remove-optional-tags --remove-redundant-attributes --remove-tag-whitespace --use-short-doctype --remove-attribute-quotes --remove-script-type-attributes  $1 > $1.tmp
rm $1;
sed -e "s/herf=/href=/g" $1.tmp > $1;
rm $1.tmp;
