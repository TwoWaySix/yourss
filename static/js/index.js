var refreshfeedButton = document.getElementById("btn-refreshfeed");
refreshfeedButton.addEventListener("click", refreshFeed);

var downloadButton = document.getElementById("btn-download");
downloadButton.addEventListener("click", download);


function refreshFeed() {
    console.log("Hello. You clicked the refresh feed button.");

    const request = new Request('http://192.168.178.103:8881/update/feed');
    
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


function download() {
    console.log("Hello. You clicked the download button.");

    const urlField = document.getElementById("vid-id");
    const videoId = urlField.value;

    const url = "http://192.168.178.103:8882/download/from/youtube/" + videoId;
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
