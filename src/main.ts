import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';

redir()

listen('redirect', (e) => {
    invoke('redirect', { to: e.payload }).then(() => {
        console.log("[redirect] -> " + e.payload)
    }).catch(err => console.error("[invoke error] -> " + err))
}).catch(err => console.error("[listener](redirect)" + err))



async function redir() {

    let configfound: boolean | unknown = await invoke('checkconfig').catch(err => console.error("[configcheck] " + err))

    try {

        await fetch('http://localhost:3000').then(res => {
            console.log(res)
            if (res.status != 404) {

                if (!configfound) {
                    invoke('redirect', { to: 'intro' }); return
                } else {
                    invoke('redirect', { to: 'create' }); return
                }
            }
        })

    } catch (error) {
        console.error("[redir] ", error)
        redir()
    }
}
