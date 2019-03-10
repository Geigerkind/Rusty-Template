html-minifier --case-sensitive --collapse-whitespace --remove-comments --remove-optional-tags --remove-redundant-attributes --remove-script-type-attributes --remove-tag-whitespace --use-short-doctype --remove-attribute-quotes --remove-script-type-attributes  $1 > $1.tmp
rm $1;
mv $1.tmp $1;