var downloadButton = document.getElementById("btn-download");
downloadButton.addEventListener("click", download);


function download() {
    console.log("Hello. You clicked the download button.");

    const videoId = urlField.value;

    const url = "http://localhost:8882/download/from/youtube/" + videoId;
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
