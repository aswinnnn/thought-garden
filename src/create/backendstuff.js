// import { emit, listen } from '@tauri-apps/api/event'
// const { listen } = require('@tauri-apps/api/event');

async function loadlisteners() {
    try {

        await window.__TAURI__.event.listen('fill_post', (event) => {
            let title = document.querySelector('.article-title');
            let content = document.querySelector('.article-content');
            let date = document.querySelector('.date');
            let article = document.querySelector('.article');


            console.log(event.payload)

            title.innerHTML = event.payload.buffer_title;
            content.innerText = event.payload.buffer;
            date.innerText = event.payload.metadata.created_at;
            console.log(article)
            article.setAttribute('data-id', event.payload.uuid_str);
        })


        await window.__TAURI__.event.listen('change_style', (event) => {
            document.querySelector('#body').setAttribute('style', 'background-image: url(\'' + event.payload + '\')');
        })

        // input date through metadata
        // perfomance issues

        console.log('listeners active')
    } catch (error) {
        console.error("[fill_post_listener] ", error)
    }

}
// emits the `click` event with the object payload
// emit('click', {
//   theMessage: 'Tauri is awesome!',
// })

async function fill_post(postId) {
    try {
        await window.__TAURI__.invoke('fill_post', { postId: postId });
    } catch (error) {
        console.error('Error invoking Tauri fill_post[home.rs:46]:', error);
    }
}