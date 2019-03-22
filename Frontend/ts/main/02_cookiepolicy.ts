declare var getCookie: any;
if (getCookie("cookieconsent") == null)
{
    lazyloadCSS("/external/cookiepolicy/cookieconsent.min.css");
    lazyloadJS("/external/cookiepolicy/cookieconsent.min.js");
}