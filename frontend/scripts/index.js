var historyContainer = document.querySelector("#history #container");
var currencyFormatter = Intl.NumberFormat("en-MY", {
    style: 'currency',
    currency: 'MYR'
})

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
    },
    {
        amount: 1800.00,
        type: "incoming",
        timestamp: 1753369285000,
        party: "salary_payment"
    },
    {
        amount: 299.99,
        type: "outgoing",
        timestamp: 1753319285000,
        party: "online_store"
    },
    {
        amount: 125.00,
        type: "incoming",
        timestamp: 1753269285000,
        party: "refund_dept"
    },
    {
        amount: 67.25,
        type: "outgoing",
        timestamp: 1753219285000,
        party: "gas_station"
    },
    {
        amount: 3200.00,
        type: "incoming",
        timestamp: 1753169285000,
        party: "bonus_payment"
    },
    {
        amount: 180.75,
        type: "outgoing",
        timestamp: 1753119285000,
        party: "restaurant"
    },
    {
        amount: 95.50,
        type: "incoming",
        timestamp: 1753069285000,
        party: "cashback_reward"
    },
    {
        amount: 520.00,
        type: "outgoing",
        timestamp: 1753019285000,
        party: "rent_payment"
    },
    {
        amount: 340.25,
        type: "incoming",
        timestamp: 1752969285000,
        party: "side_gig"
    },
    {
        amount: 78.90,
        type: "outgoing",
        timestamp: 1752919285000,
        party: "pharmacy"
    },
    {
        amount: 1100.00,
        type: "incoming",
        timestamp: 1752869285000,
        party: "investment_return"
    },
    {
        amount: 245.60,
        type: "outgoing",
        timestamp: 1752819285000,
        party: "utility_bill"
    },
    {
        amount: 85.00,
        type: "incoming",
        timestamp: 1752769285000,
        party: "gift_money"
    },
    {
        amount: 199.99,
        type: "outgoing",
        timestamp: 1752719285000,
        party: "electronics_store"
    }
];

function populateContainer() {
    let result = '';

    transactions.forEach((e) => {
        let time = new Date(e.timestamp).toLocaleString().split(', ');

        result += `<div class="transaction">
            <div>
                <img src='./assets/${e.type}.png'/>
                <div>
                    <h3>
                        ${currencyFormatter.format(e.amount)}
                    </h3>
                    <h4>
                        ${e.type == 'outgoing' ? 'to' : 'from'} ${e.party}
                    </h4>
                </div>
            </div>
            <div id="date-section">
                <h4>${time[1]}</h4>
                <h4>${time[0]}</h4>
            </div>
        </div>`
    })

    historyContainer.innerHTML = result;
}

populateContainer();
