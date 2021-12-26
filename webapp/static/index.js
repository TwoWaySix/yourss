console.log("Test1")
const button = document.getElementById('download');
console.log(button);
console.log("Test2");

button.addEventListener('click', async _ => {
  
  const response = await fetch('http://192.168.178.103:8765/api/mp3', {
    method: 'get',
    mode: 'same-origin' // cors, no-cors, *cors, same-origin
  });
  const j = response.json();   
  
});