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
    // 'receive': document.querySelector("#receive-container"),
    'pending': document.querySelector("#pending-container"),
    'status': document.querySelector("#status-container")
}

const usernameInputEl = containerEls.send.querySelector('input#username-input');
const amountInputEl = containerEls.send.querySelector('input#amount-input');

const statusHeaderEl = containerEls.status.querySelector('#header');

var activeMode = '';
var isInputValid = false;
var transactions = [
    {
        amount: 500.00,
        type: "incoming",
        timestamp: 1753769285000,
        party: "john_doe"
    },
    {
        amount: 423.55,
        type: "outgoing",
        timestamp: 1753719285000,
        party: "mary_jane"
    },
    {
        amount: 1250.75,
        type: "incoming",
        timestamp: 1753669285000,
        party: "alice_smith"
    },
    {
        amount: 89.99,
        type: "outgoing",
        timestamp: 1753619285000,
        party: "bob_wilson"
    },
    {
        amount: 2000.00,
        type: "incoming",
        timestamp: 1753569285000,
        party: "company_xyz"
    },
    {
        amount: 156.30,
        type: "outgoing",
        timestamp: 1753519285000,
        party: "grocery_store"
    },
    {
        amount: 750.50,
        type: "incoming",
        timestamp: 1753469285000,
        party: "freelance_client"
    },
    {
        amount: 45.00,
        type: "outgoing",
        timestamp: 1753419285000,
        party: "coffee_shop"
    }
];

function renderTransactions() {
    historyContainerEl.innerHTML = '';

    transactions.forEach((transaction) => {
        renderTransaction(transaction);
    })
}

function renderTransaction(transaction) {
    let transactionEl = transactionTemplateEl.cloneNode(true);

    let dateTimeParts = new Date(transaction.timestamp).toLocaleString().split(', ');

    transactionEl.querySelector('#icon').src = `./assets/${transaction.type}.png`;
    transactionEl.querySelector('#amount').innerHTML = currencyFormatter.format(transaction.amount);
    transactionEl.querySelector('#type').innerHTML = `${transaction.type == 'outgoing' ? 'to' : 'from'} ${transaction.party}`;
    transactionEl.querySelector('#date').innerHTML = dateTimeParts[0];
    transactionEl.querySelector('#time').innerHTML = dateTimeParts[1];

    historyContainerEl.appendChild(transactionEl);
}

function setActiveMode(newMode) {
    activeMode = newMode;

    updateContainerStates();
}

function updateContainerStates() {
    for (const [key, el] of Object.entries(containerEls)) {
        el.ariaLabel = activeMode == key ? '' : 'disabled';
    };
}

// #region send
function sendMoney() {
    if (!isInputValid) {
        return;
    }

    setActiveMode('pending');

    setTimeout(() => {
        setActiveMode('status');
    }, 1000);
    // send request
    // close menu
    // update transaction list
    // update balance
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
updateContainerStates();
validateSendInputs();
