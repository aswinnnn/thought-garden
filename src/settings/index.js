(async function () {
    
    await window.__TAURI__.event.listen('settings_unload', async (event) => {
       console.log(event.payload) 
    })

})()
