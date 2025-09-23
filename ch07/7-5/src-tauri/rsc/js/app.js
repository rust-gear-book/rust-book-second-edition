const { event } = window.__TAURI__;

event.listen("file-list", (event) => {
    let fileListElement = document.getElementById("file-list");
    fileListElement.innerHTML = ""; // clear child elements

    let ulElement = document.createElement("ul");
    for (var i = 0; i < event.payload.entryList.length; i++) {
        let entry = event.payload.entryList[i];
        console.log(entry.typeName + " " + entry.filePath);

        let liElement = document.createElement("li");
        liElement.innerHTML = entry.filePath;

        liElement.classList.add("entry");
        ulElement.appendChild(liElement);
    }
    fileListElement.appendChild(ulElement);
});
