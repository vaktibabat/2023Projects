//Add a tab's name to the list in the document
function addNodeToList(tabName, tabObj) {
    var node = document.createElement('li');
    //Add the button
    let closeBtn = document.createElement('button');
    closeBtn.innerHTML = 'Close Tab';
    closeBtn.addEventListener("click", function() {
        browser.tabs.remove(tabObj.id);        
    })
    
    node.appendChild(document.createTextNode(tabName));
    node.appendChild(closeBtn)

    document.querySelector('ul').appendChild(node);
}

//Empties the tab list in the document
function emptyList() {
    const tabList = document.getElementById("sound-tabs");
    tabList.innerHTML = '';
}

//Changes the list according to the current audible nodes
function changeList(tabs) {
    emptyList();    

    for (const tab of tabs) {
       addNodeToList(tab.title, tab);
    }
}

//Consol.error()s an error
function onError(error) {
    console.error('Error: ${error}');
}

//Query current nodes
browser.tabs.query({ audible: true }).then(changeList, onError);
