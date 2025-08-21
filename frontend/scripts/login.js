const containerEl = document.querySelector("#container");

const toggleButtonEl = document.querySelector("#toggle-button");
const submitButtonEl = document.querySelector("#submit-button");

const usernameInputEl = document.querySelector("#username-input");
const passwordInputEl = document.querySelector("#password-input");
const confirmPasswordInputEl = document.querySelector("#confirm-password-input");

const statusMessageEl = document.querySelector("#status-message");
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
    statusMessageEl.innerHTML = '';
}

function updateSubmitButtonState() {
    submitButtonEl.ariaLabel = isInputValid ? '' : 'disabled';
}
// #endregion

// #region handlers
async function login(username, password) {
    fetch('/auth/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        },
        body: JSON.stringify({ username, password })
    })
        .then((res) => {
            if (res.status == 403) {
                statusMessageEl.innerHTML = 'Invalid credentials';
                return null;
            }

            return res.json();
        })
        .then((data) => {
            if (!data) {
                return;
            }

            if (!data?.id || !data?.access_token) {
                statusMessageEl.innerHTML = 'Unparseable login body';
                return;
            }

            localStorage.setItem('id', data.id);
            localStorage.setItem('username', username);
            localStorage.setItem('access_token', data.access_token);

            window.location.href = '/index.html';
        }).catch((err) => {
            statusMessageEl.innerHTML = 'Network error';
        })
}

async function signup(username, password) {
    fetch('/auth/signup', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json'
        },
        body: JSON.stringify({ username, password })
    })
        .then((res) => {
            if (res.status === 400) {
                statusMessageEl.innerHTML = 'Username taken';
                return null;
            }

            if (!res.ok) {
                statusMessageEl.innerHTML = 'Error occured';
                return null;
            }

            return res.json();
        })
        .then((data) => {
            if (!data) {
                return;
            }

            // login with these credentials, that we know are valid and legal
            login(username, password);
        })
        .catch((err) => {
            console.log(err);
            statusMessageEl.innerHTML = 'Network error';
        })
}
// #endregion

document.querySelector("#submit-button").removeEventListener?.("click", () => { }); // no-op to avoid duplicates if reloaded
submitButtonEl.addEventListener("click", async () => {
    if (!isInputValid) {
        return;
    }

    const username = usernameInputEl.value;
    const password = passwordInputEl.value;

    await (activeMode === 'login' ? login : signup)(username, password);
})

updateSubmitButtonState();
usernameInputEl.addEventListener('keyup', validateAuthInputs);
passwordInputEl.addEventListener('keyup', validateAuthInputs);
confirmPasswordInputEl.addEventListener('keyup', validateAuthInputs);
validateAuthInputs();
