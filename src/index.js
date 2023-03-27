const { invoke } = window.__TAURI__.tauri
const { listen } = window.__TAURI__.event

const rarityMap = {
  Rarity_Rare_Weapon: 'bl',
  Rarity_Mythical_Weapon: 'pu',
  Rarity_Legendary_Weapon: 'pi',
  Rarity_Ancient_Weapon: 're',
  Rarity_Ancient: 'go' 
}

const wearMap = {
  'Factory New': 'fn',
  'Minimal Wear': 'mw',
  'Field-Tested': 'ft',
  'Well-Worn': 'ww',
  'Battle-Scarred': 'bs'
}

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

      process_stats(payload)

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

async function process_stats(data) {
  const rarityCount = {}
  const conditionCount = {}
  let stattrakCount = 0
  let caseCount = data.filter(d => !d.result.includes('Sticker')).length

  // Get counts for each rarity and condition
  for (chunk of data) {
    if (rarityCount[chunk.rarity.rarity]) {
      rarityCount[chunk.rarity.rarity]++
    } else {
      rarityCount[chunk.rarity.rarity] = 1
    }

    if (conditionCount[chunk.rarity.condition]) {
      conditionCount[chunk.rarity.condition]++
    } else {
      conditionCount[chunk.rarity.condition] = 1
    }

    if (chunk.result.includes('StatTrak')) stattrakCount++
  }

  // Get averages for rarities and qualities
  for (rarity in rarityCount) {
    rarityCount[rarity] = (rarityCount[rarity] / caseCount * 100).toFixed(2)
  }

  for (condition in conditionCount) {
    conditionCount[condition] = (conditionCount[condition] / caseCount * 100).toFixed(2)
  }

  // Get stattrak percentage
  const stattrakAvg = (stattrakCount / caseCount * 100).toFixed(2)

  // Set text for each rarity
  for (rarity of Object.keys(rarityCount)) {
    const rarityText = document.querySelector(`#${rarityMap[rarity]} .result`)
    rarityText.innerHTML = `${rarityCount[rarity]}%`
  }

  for (cond of Object.keys(conditionCount)) {
    const condText = document.querySelector(`#${wearMap[cond]} .result`)
    condText.innerHTML = `${conditionCount[cond]}%`
  }

  const avgs = document.querySelector('#averages')
  avgs.classList.remove('hide')

  console.log(rarityCount)
  console.log(conditionCount)
  console.log(stattrakCount)
}