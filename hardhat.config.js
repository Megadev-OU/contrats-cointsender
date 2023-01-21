require("dotenv").config();
require("@nomicfoundation/hardhat-toolbox");
require('@openzeppelin/hardhat-upgrades');
require("@nomiclabs/hardhat-etherscan");

const forkingAccountsBalance = `50000${"0".repeat(18)}`;


let realAccounts = [
  {
    privateKey: `0x${process.env.PRIVATE_KEY}`,
    balance: forkingAccountsBalance,
  },
];

module.exports = {
    solidity: "0.8.17",
    networks: {
        testnet: {
            url: "https://data-seed-prebsc-1-s3.binance.org:8545",
            chainId: 97,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        bsc: {
            url: "https://bsc-dataseed.binance.org/",
            chainId: 56,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        avalanche: {
            url: "https://api.avax.network/ext/bc/C/rpc",
            chainId: 43114,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        polygon: {
            url: "https://polygon-rpc.com/",
            chainId: 137,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        eth: {
            url: "https://mainnet.infura.io/v3/",
            chainId: 1,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        godwoken: {
            url: "https://mainnet.godwoken.io/rpc/eth-wallet",
            chainId: 71394,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        arbitrum: {
            url: "https://arb1.arbitrum.io/rpc",
            chainId: 42161,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        optimism: {
            url: "https://optimism-mainnet.infura.io",
            chainId: 10,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        aurora: {
            url: "https://aurora-mainnet.infura.io",
            chainId: 1313161554,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        fantom: {
            url: "https://rpc.ankr.com/fantom/",
            chainId: 250,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        arbitrumOne: {
            url: "https://rpc.ankr.com/arbitrum",
            chainId: 42161,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        celo: {
            url: "https://forno.celo.org",
            chainId: 42220,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        moonbeam: {
            url: "https://rpc.api.moonbeam.network",
            chainId: 1284,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        ronin: {
            url: "https://api.roninchain.com/rpc",
            chainId: 2020,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        gnosis: {
            url: "https://rpc.gnosischain.com",
            chainId: 100,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        fuse: {
            url: "https://rpc.fuse.io",
            chainId: 122,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        shardeum: {
            url: "https://liberty10.shardeum.org",
            chainId: 8080,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        quai: {
            url: "https://quai.node.endpoint.io",
            chainId: 994,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        arbitrumNova: {
            url: "https://nova.arbitrum.io/rpc",
            chainId: 42170,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        zkSync: {
            url: "https://zksync2-testnet.zksync.dev",
            chainId: 280,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        scroll: {
            url: "https://prealpha-rpc.scroll.io/l1",
            chainId: 534351,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        scrollL2: {
            url: "https://prealpha-rpc.scroll.io/l2",
            chainId: 534354,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },

        hardhat: {
            accounts: realAccounts,
            chainId: 1337,
        },
    },
    paths: {
        sources: "./contracts",
        tests: "./test",
        cache: "./cache",
        artifacts: "./artifacts"
    },
    mocha: {
        timeout: 20000
    },
    etherscan: {
        apiKey: process.env.ETHERSCAN_API_KEY
    },
};
