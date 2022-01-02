var downloadButton = document.getElementById("btn-download");
downloadButton.addEventListener("click", download);


function download() {
    console.log("Hello. You clicked the download button.");

    const urlField = document.getElementById("vid-id");
    const videoId = urlField.value;

    const downloaderIp = document.getElementById('server-ip').value;

    const url = "http://" + downloaderIp + "/download/from/youtube/" + videoId;
    console.log(url);
    const request = new Request(url);
    
    fetch(request)
        .then(response => {
            if (response.status === 200) {
                return response.json();
            } else {
                throw new Error('Something went wrong on api server!');
            }
        })
        .then(response => {
            console.debug(response);
            // ...
        }).catch(error => {
            console.error(error);
        });
}
