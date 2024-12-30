(async function () {
    let conf = (await (await fetch('/api/settings')).json());
    window.settings = conf;

    document.querySelectorAll(".switch__label").forEach((e) => {
        if (e.getAttribute("for")=="one") {
            e.innerHTML = conf["home.emoji"]
        }
        else {
            e.innerHTML = conf["create.emoji"]
        }
    })
})()