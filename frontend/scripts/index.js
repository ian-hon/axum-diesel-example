const transactionTemplateEl = (new DOMParser()).parseFromString(`<div class="transaction">
    <div>
        <img id="icon" src=''/>
        <div>
            <h3 id="amount"></h3>
            <h4 id="type"></h4>
        </div>
    </div>
    <div id="date-section">
        <h4 id="date"></h4>
        <h4 id="time"></h4>
    </div>
</div>`, 'text/html').documentElement.querySelector(".transaction");

var currencyFormatter = Intl.NumberFormat("en-MY", {
    style: 'currency',
    currency: 'MYR'
})

const historyContainerEl = document.querySelector("#history #container");
const containerEls = {
    'send': document.querySelector("#send-container"),
    'receive': document.querySelector("#receive-container"),
    'pending': document.querySelector("#pending-container"),
    'status': document.querySelector("#status-container")
}

const usernameInputEl = containerEls.send.querySelector('input#uuid-input');
const amountInputEl = containerEls.send.querySelector('input#amount-input');

const statusHeaderEl = containerEls.status.querySelector('#header');

var activeMode = '';
var isInputValid = false;

var uuid = localStorage.getItem('id');
var username = localStorage.getItem('username');
var accessToken = localStorage.getItem('access_token');

if (!uuid || !username || !accessToken) {
    window.location.href = 'login.html';
}
document.querySelector('#username').innerHTML = username;
document.querySelector('#uuid').innerHTML = uuid;

function renderBalance() {
    fetch(`/users/${uuid}`, {
        headers: {
            'Authorization': `Bearer ${accessToken}`,
            'Accept': 'application/json'
        }
    })
        .then((r) => {
            if ((r.status === 401) || (r.status === 403)) {
                window.location.href = 'login.html';
                return null;
            }

            if (!r.ok) {
                return null;
            }
            return r.json();
        })
        .then((data) => {
            if (!data) return;

            let amount = Number(data.balance);
            document.querySelector('#balance').innerHTML = currencyFormatter.format(isNaN(amount) ? 0 : amount);
        })
        .catch((err) => {
            console.log(err);
            window.location.href = 'login.html';
        });
}

function renderTransactions() {
    historyContainerEl.innerHTML = '';

    transactions.forEach((transaction) => {
        // id: Uuid,
        // amount: BigDecimal,
        // recipient: Uuid,
        // sender: Uuid,
        // timestamp: jiff::Timestamp,

        let transactionEl = transactionTemplateEl.cloneNode(true);

        let dateTimeParts = new Date(transaction.timestamp).toLocaleString().split(', ');

        transactionEl.querySelector('#icon').src = `./assets/${transaction.sender === uuid ? 'outgoing' : 'incoming'}.png`;
        // transactionEl.querySelector('#icon').src = `./assets/${transaction.type}.png`;
        transactionEl.querySelector('#amount').innerHTML = currencyFormatter.format(transaction.amount);
        // transactionEl.querySelector('#type').innerHTML = `${transaction.type == 'outgoing' ? 'to' : 'from'} ${transaction.party}`;
        transactionEl.querySelector('#type').innerHTML = transaction.sender === uuid ? `to ${transaction.recipient}` : `from ${transaction.sender}`;
        transactionEl.querySelector('#date').innerHTML = dateTimeParts[0];
        transactionEl.querySelector('#time').innerHTML = dateTimeParts[1];

        historyContainerEl.appendChild(transactionEl);
    })
}

function renderReceiveQr(text) {
    const qrEl = containerEls.receive.querySelector('#qr-code');
    if (!qrEl) return;

    qrEl.innerHTML = '';

    const styles = getComputedStyle(document.documentElement);

    new QRCode(qrEl, {
        text,
        width: 200,
        height: 200,
        colorDark: styles.getPropertyValue('--background'),
        colorLight: '#fff',
        correctLevel: QRCode.CorrectLevel.M
    });
}

function setActiveMode(newMode) {
    activeMode = newMode;

    updateContainerStates();

    if (newMode === 'receive') {
        // Redirect link with recipient UUID as a query parameter
        const qrLink = `${location.origin}/index.html?to=${encodeURIComponent(uuid)}`;
        renderReceiveQr(qrLink);
    }
}

function updateContainerStates() {
    for (const [key, el] of Object.entries(containerEls)) {
        el.ariaLabel = activeMode == key ? '' : 'disabled';
    };
}

// pub struct PostTranscactionPayload {
//     #[serde(with = "bigdecimal::serde::json_num")]
//     amount: BigDecimal,
//     recipient: Uuid,
//     sender: Uuid,
// }

// #region send
function sendMoney() {
    if (!isInputValid) {
        return;
    }

    let amount = Number(amountInputEl.value);
    let recipient = containerEls.send.querySelector('input#uuid-input')?.value;
    if (isNaN(amount) || (amount <= 0) || !recipient) {
        isInputValid = false;
        return;
    }

    setActiveMode('pending');

    fetch(`/transactions/`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${accessToken}`,
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            amount: amount,
            recipient: recipient,
            sender: uuid
        })
    })
        .then((res) => {
            console.log(res);
            statusHeaderEl.innerHTML = res.ok ? 'success!' : 'error';
            setActiveMode('status');
        })
        .catch((err) => {
            console.log(err);
            statusHeaderEl.innerHTML = 'connection<br>error';
            setActiveMode('status');
        });

    renderBalance();
    renderTransactions();
}

function validateSendInputs() {
    isInputValid = !!usernameInputEl.value && !!amountInputEl.value;
    isInputValid = isInputValid && (parseFloat(amountInputEl.value) > 0);
    isInputValid = isInputValid && (parseFloat(amountInputEl.value) > 0);

    containerEls.send.querySelector('#action #send').ariaLabel = isInputValid ? '' : 'disabled';
}
// #endregion

// if session key invalid/doesnt exist, redirect to login

usernameInputEl.addEventListener('keyup', validateSendInputs);
amountInputEl.addEventListener('keyup', validateSendInputs);
renderTransactions();
renderBalance();
updateContainerStates();
validateSendInputs();

(function initRecipientFromQuery() {
    try {
        const to = new URL(window.location.href).searchParams.get('to');
        if (to) {
            const input = containerEls.send.querySelector('input#uuid-input');
            if (input) {
                input.value = to;
                setActiveMode('send');
                const cleanUrl = `${location.origin}${location.pathname}${location.hash}`;
                history.replaceState({}, '', cleanUrl);
            }
        }
    } catch (_) { }
})();

// setInterval(() => {
//     // horrible, yes
//     // no ws or sse though so this is the next simplest alternative
//     renderBalance();
//     renderTransactions();
// }, 2000);
