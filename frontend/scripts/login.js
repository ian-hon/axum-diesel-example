const usernameField = document.querySelector("#username-input");
const passwordField = document.querySelector("#password-input");

function login(username, password) {

}

function signup(username, password) {

}

document.querySelector("#submit-button").addEventListener("click", () => {
    const username = usernameField.value;
    const password = passwordField.value;

    localStorage.setItem('username', username);
    localStorage.setItem('password', sha256(password));

    window.location.href = '/index.html';
})
