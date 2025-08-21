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

const usernameInputEl = containerEls.send.querySelector('input#username-input');
const amountInputEl = containerEls.send.querySelector('input#amount-input');

const statusHeaderEl = containerEls.status.querySelector('#header');

var activeMode = '';
var isInputValid = false;

var uuid = localStorage.getItem('id');
var username = localStorage.getItem('username');
var accessToken = localStorage.getItem('access_token');
var transactions = [
    {
        amount: 500.00,
        type: "incoming",
        timestamp: 1753769285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 423.55,
        type: "outgoing",
        timestamp: 1753719285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 1250.75,
        type: "incoming",
        timestamp: 1753669285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 89.99,
        type: "outgoing",
        timestamp: 1753619285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 2000.00,
        type: "incoming",
        timestamp: 1753569285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 156.30,
        type: "outgoing",
        timestamp: 1753519285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 750.50,
        type: "incoming",
        timestamp: 1753469285000,
        party: "917239154618920879102675647152973871293"
    },
    {
        amount: 45.00,
        type: "outgoing",
        timestamp: 1753419285000,
        party: "917239154618920879102675647152973871293"
    }
];

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

        // transactionEl.querySelector('#icon').src = `./assets/${transaction.sender === uuid ? 'outgoing' : 'incoming'}.png`;
        transactionEl.querySelector('#icon').src = `./assets/${transaction.type}.png`;
        transactionEl.querySelector('#amount').innerHTML = currencyFormatter.format(transaction.amount);
        transactionEl.querySelector('#type').innerHTML = `${transaction.type == 'outgoing' ? 'to' : 'from'} ${transaction.party}`;
        // transactionEl.querySelector('#type').innerHTML = transaction.sender === uuid ? `to ${transaction.recipient}` : `from ${transaction.sender}`;
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
        renderReceiveQr(uuid);
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
    let recipient = usernameInputEl.value;
    if (isNaN(amount) || (amount <= 0) || !recipient) {
        isInputValid = false;
        return;
    }

    setActiveMode('pending');

    fetch(`/transactions/`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${accessToken}`,
            'Accept': 'application/json'
        },
        body: {
            'amount': amount,
            'recipient': recipient,
            'sender': uuid
        }
    })
        .then((res) => {
            console.log(res);
            statusHeaderEl.innerHTML = 'success?';

            // id: Uuid,
            // amount: BigDecimal,
            // recipient: Uuid,
            // sender: Uuid,
            // timestamp: jiff::Timestamp,
            setActiveMode('status');
        })
        .catch((err) => {
            console.log(err);
            statusHeaderEl.innerHTML = 'connection<br>error';
            setActiveMode('status');
        })

    // send request
    // close menu
    // update transaction list
    // update balance
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

// setInterval(() => {
//     // horrible, yes
//     // no ws or sse though so this is the next simplest alternative
//     renderBalance();
//     renderTransactions();
// }, 2000);
