const transactionComponent = (new DOMParser()).parseFromString(`<div class="transaction">
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

var historyContainer = document.querySelector("#history #container");

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

function populateContainer() {
    historyContainer.innerHTML = '';

    transactions.forEach((e) => {
        appendTransaction(e);
    })
}

function appendTransaction(incoming) {
    let newComponent = transactionComponent.cloneNode(true);

    let time = new Date(incoming.timestamp).toLocaleString().split(', ');

    newComponent.querySelector('#icon').src = `./assets/${incoming.type}.png`;
    newComponent.querySelector('#amount').innerHTML = currencyFormatter.format(incoming.amount);
    newComponent.querySelector('#type').innerHTML = `${incoming.type == 'outgoing' ? 'to' : 'from'} ${incoming.party}`;
    newComponent.querySelector('#date').innerHTML = time[0];
    newComponent.querySelector('#time').innerHTML = time[1];

    historyContainer.appendChild(newComponent);
}

// if session key invalid/doesnt exist, redirect to login

populateContainer();
