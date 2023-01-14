# Installation

```shell
npm i 
npm run compile 
```

# Network config

1. Go to hardhat.config.js

2. Update networks

```code
    networks: {
        testnet: {
            url: "https://data-seed-prebsc-1-s1.binance.org:8545",
            chainId: 97,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        mainnet: {
            url: "https://bsc-dataseed.binance.org/",
            chainId: 56,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        localhost: {
            url: "http://127.0.0.1:8545"
        }
    },
```

# Deploy

1. Set keys for deploy in .env
```code
    See structure in .env.example
```

2. Set address for owner in  deploy.js

```code
    const ownerAddress = '0x123abc'
```

3. Run command

 ```shell
   npm run deploy
```
