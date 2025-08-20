const containerEl = document.querySelector("#container");

const toggleButtonEl = document.querySelector("#toggle-button");
const submitButtonEl = document.querySelector("#submit-button");

const usernameInputEl = document.querySelector("#username-input");
const passwordInputEl = document.querySelector("#password-input");
const confirmPasswordInputEl = document.querySelector("#confirm-password-input");

const confirmPasswordStatusEl = document.querySelector("#confirm-password-status");

var activeMode = 'login';
/**
 * @see https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html#implement-proper-password-strength-controls
 */
const PASSWORD_LENGTH = 8;
var isInputValid = false;

// #region mode
function toggleAuthMode() {
    activeMode = activeMode == 'login' ? 'signup' : 'login';
    containerEl.ariaLabel = activeMode;

    toggleButtonEl.innerHTML = activeMode == 'login' ? 'sign up instead' : 'back to login';
    submitButtonEl.innerHTML = activeMode;

    validateAuthInputs();
}
// #endregion

// #region password
function validateAuthInputs() {
    isInputValid = true;

    // username
    isInputValid = !!usernameInputEl.value;

    // password
    isInputValid = isInputValid && (passwordInputEl.value.length >= PASSWORD_LENGTH);

    // confirm password
    if (activeMode == "signup") {
        isInputValid = isInputValid && (confirmPasswordInputEl.value === passwordInputEl.value);
        confirmPasswordStatusEl.ariaLabel = (confirmPasswordInputEl.value === passwordInputEl.value) ? 'disabled' : '';
    }

    updateSubmitButtonState();
}

function updateSubmitButtonState() {
    submitButtonEl.ariaLabel = isInputValid ? '' : 'disabled';
}
// #endregion


function login(username, password) {
    console.log(`attempt login : ${username} : ${password}`);
}

function signup(username, password) {
    console.log(`attempt signup : ${username} : ${password}`);
}

document.querySelector("#submit-button").removeEventListener?.("click", () => { }); // no-op to avoid duplicates if reloaded
submitButtonEl.addEventListener("click", () => {
    if (!isInputValid) {
        return;
    }

    const username = usernameInputEl.value;
    const password = passwordInputEl.value;

    // TODO: Send API request.

    console.log('redirect');

    let result = (activeMode === 'login' ? login : signup)(username, password);

    // will result be just a result? or does it return a session key?
    if (result == AuthResult.SUCCESS) {
        // store session key and redirect
        // window.location.href = './index.html';
    } else {

    }
})

const AuthResult = Object.freeze({
    SUCCESS: 'success',
    INVALID_CREDENTIALS: 'invalid_credentials'
})


updateSubmitButtonState();
usernameInputEl.addEventListener('keyup', validateAuthInputs);
passwordInputEl.addEventListener('keyup', validateAuthInputs);
confirmPasswordInputEl.addEventListener('keyup', validateAuthInputs);
validateAuthInputs();
