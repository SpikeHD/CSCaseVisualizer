document.addEventListener('DOMContentLoaded', () => {
  const btn = document.querySelector('button')
  const cookie = document.querySelector('#cookie')

  btn.addEventListener('click', () => {
    btn.classList.add('clicked')

    window.location.href = `pages/calculate.html?cookie=${cookie.value}`
  })
})