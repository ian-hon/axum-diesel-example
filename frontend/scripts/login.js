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
function login(username, password) {
    console.log(`attempt login : ${username} : ${password}`);
    return (async () => {
        try {
            const loginRes = await fetch('/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                },
                body: JSON.stringify({ username, password })
            });

            if (loginRes.status == 403) {
                statusMessageEl.innerHTML = 'Invalid credentials';
                return SignupResponse.ERROR;
            }

            const loginData = await loginRes.json().catch(() => ({}));
            if (!loginData?.id || !loginData?.access_token) {
                statusMessageEl.innerHTML = 'Unparseable login body';
                return SignupResponse.ERROR;
            }

            localStorage.setItem('id', loginData.id);
            localStorage.setItem('username', username);
            localStorage.setItem('access_token', loginData.access_token);

            window.location.href = '/index.html';
            return SignupResponse.SUCCESS;
        } catch (err) {
            // unset loading
            // console.error('signup error:', err);
            statusMessageEl.innerHTML = 'Network error';
            return SignupResponse.ERROR;
        }
    })();
}

function signup(username, password) {
    // {id: "0198c861-cf82-79a2-9bf6-aa059fb6df6a"}
    return (async () => {
        try {
            const res = await fetch('/auth/signup', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                },
                body: JSON.stringify({ username, password })
            });

            if (res.status === 400) {
                // username taken
                statusMessageEl.innerHTML = 'Username taken';
                return SignupResponse.USERNAME_TAKEN;
            }

            if (!res.ok) {
                statusMessageEl.innerHTML = 'Error occured';
                return SignupResponse.ERROR;
            }

            // login with these credentials, that we know are valid and legal
            login(username, password);

            window.location.href = '/index.html';
            return SignupResponse.SUCCESS;
        } catch (err) {
            // unset loading
            // console.error('signup error:', err);
            statusMessageEl.innerHTML = 'Network error';
            return SignupResponse.ERROR;
        }
    })();
}
// #endregion

document.querySelector("#submit-button").removeEventListener?.("click", () => { }); // no-op to avoid duplicates if reloaded
submitButtonEl.addEventListener("click", async () => {
    if (!isInputValid) {
        return;
    }

    const username = usernameInputEl.value;
    const password = passwordInputEl.value;

    (activeMode === 'login' ? login : signup)(username, password);
})

const SignupResponse = Object.freeze({
    SUCCESS: 'success',
    USERNAME_TAKEN: 'username_taken',
    ERROR: 'error'
})


updateSubmitButtonState();
usernameInputEl.addEventListener('keyup', validateAuthInputs);
passwordInputEl.addEventListener('keyup', validateAuthInputs);
confirmPasswordInputEl.addEventListener('keyup', validateAuthInputs);
validateAuthInputs();
