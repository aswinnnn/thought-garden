function loadsettings() {
    (async function () {
        let conf = (await (await fetch('/api/settings')).json());
        window.settings = conf;

        document.querySelectorAll(".switch__label").forEach((e) => {
            if (e.getAttribute("for") == "one") {
                e.innerHTML = conf["home.emoji"]
            }
            else {
                e.innerHTML = conf["create.emoji"]
            }
        })

        const body = document.querySelector('body');
        body.style.setProperty('--light-bg', conf["theme.light.background-color"]);
        body.style.setProperty('--light-article-bg', conf["theme.light.secondary-color"]);
        if (body.className=="light") {
            body.style.backgroundColor = conf["theme.light.background-color"];
            body.style.color = conf["theme.light.text-color"];
        } else {
            body.style.backgroundColor = conf["theme.dark.background-color"];
            body.style.color = conf["theme.dark.text-color"];            
        }
        body.style.setProperty('--light-text', conf["theme.light.text-color"]);
        body.style.setProperty('--dark-bg', conf["theme.dark.background-color"]);
        body.style.setProperty('--dark-article-bg', conf["theme.dark.secondary-color"]);
        body.style.setProperty('--dark-text', conf["theme.dark.text-color"]);

    })()

}

loadsettings()