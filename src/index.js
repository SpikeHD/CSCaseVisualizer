const { invoke } = window.__TAURI__.tauri
const { listen } = window.__TAURI__.event

document.addEventListener('DOMContentLoaded', () => {
  const btn = document.querySelector('button')
  const main = document.querySelector('#main')
  const calc = document.querySelector('#calculate')
  const cookie = document.querySelector('#cookie')

  const prog = document.querySelector('#progress')
  const loading = document.querySelector('#loading')

  btn.addEventListener('click', () => {
    btn.classList.add('clicked')
    
    main.classList.add('hide')
    calc.classList.remove('hide')

    console.log(cookie.value)

    // Invoke tauri method to begin processing
    invoke('get_main', {
      cookie: cookie.value
    })

    // Listen for tauri page process
    listen('page_process', ({ payload }) => {
      prog.innerHTML = `${payload} pages...`
    })

    listen('finish_process', ({ payload }) => {
      prog.classList.add('hide')
      loading.classList.add('hide')
      calc.classList.add('showResults')

      showHistory(payload)

      console.log(payload)
    })
  })
})

async function showHistory(data) {
  const history = document.querySelector('#history')
  history.classList.remove('hide')

  for (chunk of data) {
    const row = document.createElement('div')
    row.classList.add('row')

    const box = document.createElement('div')
    const boxImg = document.createElement('img')
    const boxName = document.createElement('div')
    box.classList.add('cell')
    boxImg.src = chunk.case_img
    boxName.innerHTML = chunk.case

    box.appendChild(boxImg)
    box.appendChild(boxName)
    
    const arrow = document.createElement('div')
    arrow.classList.add('cell')

    const arrowText = document.createElement('div')
    arrowText.innerHTML = '->'
    arrow.appendChild(arrowText)

    const item = document.createElement('div')
    const itemImg = document.createElement('img')
    const itemName = document.createElement('div')
    item.classList.add('cell')
    itemImg.src = chunk.result_img
    itemName.innerHTML = chunk.result

    item.appendChild(itemImg)
    item.appendChild(itemName)

    row.appendChild(box)
    row.appendChild(arrow)
    row.appendChild(item)

    history.appendChild(row)
  }
}