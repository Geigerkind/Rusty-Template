declare var cssPath: string;

function lazyloadCSS(url: string)
{
    let gos = document.createElement('link');
    gos.rel = 'stylesheet';
    gos.href = url;
    gos.type = "text/css";
    let godefer = document.getElementsByTagName('head')[0];
    godefer.appendChild(gos);
}

function lazyloadJS(url: string)
{
    let gos = document.createElement('script');
    gos.src = url;
    gos.type = "text/javascript";
    let godefer = document.getElementsByTagName('head')[0];
    godefer.appendChild(gos);
}

// Will be inlined
lazyloadCSS(cssPath);