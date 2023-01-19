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
        mainnet: {
            url: "https://bsc-dataseed.binance.org/",
            chainId: 56,
            gasPrice: 20000000000,
            accounts: [process.env.PRIVATE_KEY]
        },
        localhost: {
            url: "http://127.0.0.1:8545"
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
