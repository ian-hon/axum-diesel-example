const container = document.querySelector("#container");

const toggleButton = document.querySelector("#toggle-button");
const submitButton = document.querySelector("#submit-button");

const usernameField = document.querySelector("#username-input");
const passwordField = document.querySelector("#password-input");
const confirmPasswordField = document.querySelector("#confirm-password-input");

const confirmPasswordStatus = document.querySelector("#confirm-password-status");

var mode = 'login';
/**
 * @see https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html#implement-proper-password-strength-controls
 */
const PASSWORD_LENGTH = 8;
var inputValidity = false;

// #region mode
function toggleMode() {
    mode = mode == 'login' ? 'signup' : 'login';
    container.ariaLabel = mode;

    toggleButton.innerHTML = mode == 'login' ? 'sign up instead' : 'back to login';
    submitButton.innerHTML = mode;

    validateInputs();
}
// #endregion

// #region password
function validateInputs() {
    inputValidity = true;

    // username
    inputValidity = !!usernameField.value;

    // password
    inputValidity = inputValidity && (passwordField.value.length >= PASSWORD_LENGTH);

    // confirm password
    if (mode == "signup") {
        inputValidity = inputValidity && (confirmPasswordField.value === passwordField.value);
        confirmPasswordStatus.ariaLabel = (confirmPasswordField.value === passwordField.value) ? 'disabled' : '';
    }

    updateButtonStatus();
}

function updateButtonStatus() {
    submitButton.ariaLabel = inputValidity ? '' : 'disabled';
}
// #endregion


function login(username, password) {

}

function signup(username, password) {

}

document.querySelector("#submit-button").addEventListener("click", () => {
    if (!inputValidity) {
        return;
    }

    const username = usernameField.value;
    const password = passwordField.value;

    // TODO: Send API request.

    console.log('redirect');

    window.location.href = './index.html';
})

updateButtonStatus();
usernameField.addEventListener('keyup', validateInputs);
passwordField.addEventListener('keyup', validateInputs);
confirmPasswordField.addEventListener('keyup', validateInputs);
validateInputs();
