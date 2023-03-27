const { invoke } = window.__TAURI__.tauri

document.addEventListener('DOMContentLoaded', () => {
  const btn = document.querySelector('button')
  const main = document.querySelector('#main')
  const calc = document.querySelector('#calculate')
  const cookie = document.querySelector('#cookie')

  btn.addEventListener('click', () => {
    btn.classList.add('clicked')
    
    main.classList.add('hide')
    calc.classList.remove('hide')

    console.log(cookie.value)

    // Invoke tauri method to begin processing
    invoke('get_main', {
      cookie: cookie.value
    })
  })
})