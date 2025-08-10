const container = document.querySelector("#container");

const toggleButton = document.querySelector("#toggle-button");
const submitButton = document.querySelector("#submit-button");

const usernameField = document.querySelector("#username-input");
const passwordField = document.querySelector("#password-input");
const confirmPasswordField = document.querySelector("#confirm-password-input");

const confirmPasswordStatus = document.querySelector("#confirm-password-status");

var inputValidity = false;
var mode = 'login';
/**
 * @see https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html#implement-proper-password-strength-controls
 */
const passwordRules = [
    {
        description: 'at least 8 characters',
        regex: "^.{8,}$"
    },
];

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
// populate password checks
// hacky vanilla js, straightforward and simple
(() => {
    let passwordStatusContainer = document.querySelector("#password-status");
    passwordRules.forEach(e => {
        passwordStatusContainer.innerHTML += `<div class="password-check" aria-label="invalid" data-regex="${e.regex}">
            <img id="check" src="./assets/check.png">
            <img id="cross" src="./assets/cross.png">
            <h4>
                ${e.description}
            </h4>
        </div>`;
    });
})();

function validateInputs() {
    inputValidity = true;

    // username
    inputValidity = !!usernameField.value;

    // password
    document.querySelectorAll(".password-check").forEach(e => {
        let result = (new RegExp(e.dataset.regex)).test(passwordField.value);
        e.ariaLabel = result ? 'valid' : 'invalid';
        inputValidity = result ? inputValidity : false;
    });

    // confirm password
    inputValidity = inputValidity && (confirmPasswordField.value === passwordField.value);
    confirmPasswordStatus.ariaLabel = (confirmPasswordField.value === passwordField.value) ? 'disabled' : '';

    updateButtonStatus();
}

function updateButtonStatus() {
    submitButton.ariaLabel = ((!!usernameField.value && !!passwordField.value) && ((mode == 'login') || inputValidity)) ? '' : 'disabled';
}
// #endregion


function login(username, password) {

}

function signup(username, password) {

}

document.querySelector("#submit-button").addEventListener("click", () => {
    if ((mode == 'login') || inputValidity) {
        return;
    }

    const username = usernameField.value;
    const password = passwordField.value;

    // TODO: Send API request.

    window.location.href = './index.html';
})

updateButtonStatus();
usernameField.addEventListener('keyup', validateInputs);
passwordField.addEventListener('keyup', validateInputs);
confirmPasswordField.addEventListener('keyup', validateInputs);
