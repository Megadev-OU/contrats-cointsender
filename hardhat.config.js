require("dotenv").config();
require("@nomicfoundation/hardhat-toolbox");
require('@openzeppelin/hardhat-upgrades');
require("@nomiclabs/hardhat-etherscan");
require('hardhat-deploy');

const forkingAccountsBalance = `50000${"0".repeat(18)}`;


let realAccounts = [
    {
        privateKey: `${process.env.PRIVATE_KEY}`,
        balance: forkingAccountsBalance,
    },
];

module.exports = {
    defaultNetwork: "godwoken",
    namedAccounts: {
        deployer: {
            default: 0
        },
    },
    networks: {

        bscTestnet: {
            url: "https://data-seed-prebsc-1-s3.binance.org:8545",
            chainId: 97,

            accounts: [process.env.PRIVATE_KEY]
        },
        bsc: {
            url: "https://bsc-dataseed.binance.org/",
            chainId: 56,

            accounts: [process.env.PRIVATE_KEY]
        },
        avalanche: {
            url: "https://avalanche-mainnet.infura.io/v3/48b0031c00cb4a30b4edf039450af9d6",
            chainId: 43114,

            accounts: [process.env.PRIVATE_KEY]
        },
        polygon: {
            url: "https://polygon-mainnet.infura.io/v3/48b0031c00cb4a30b4edf039450af9d6",
            chainId: 137,

            accounts: [process.env.PRIVATE_KEY]
        },
        mainnet: {
            url: "https://mainnet.infura.io/v3/1e9df73b43b24ec7ac8fe5c78754e45a",
            chainId: 1,
            accounts: [process.env.PRIVATE_KEY]
        },
        godwoken: {
            url: "https://v1.mainnet.godwoken.io/rpc",
            chainId: 71402,

            accounts: [process.env.PRIVATE_KEY]
        },
        arbitrumOne: {
            url: "https://rpc.ankr.com/arbitrum",
            chainId: 42161,

            accounts: [process.env.PRIVATE_KEY]
        },
        optimism: {
            url: "https://mainnet.optimism.io",
            chainId: 10,

            accounts: [process.env.PRIVATE_KEY]
        },
        aurora: {
            url: "https://aurora-mainnet.infura.io",
            chainId: 1313161554,

            accounts: [process.env.PRIVATE_KEY]
        },
        fantom: {
            url: "https://rpc.ankr.com/fantom/",
            chainId: 250,

            accounts: [process.env.PRIVATE_KEY]
        },
        celo: {
            url: "https://celo-mainnet.infura.io/v3/48b0031c00cb4a30b4edf039450af9d6",
            chainId: 42220,

            accounts: [process.env.PRIVATE_KEY]
        },
        moonbeam: {
            url: "https://rpc.api.moonbeam.network",
            chainId: 1284,

            accounts: [process.env.PRIVATE_KEY]
        },
        ronin: {
            url: "https://api.roninchain.com/rpc",
            chainId: 2020,

            accounts: [process.env.PRIVATE_KEY]
        },
        gnosis: {
            url: "https://rpc.gnosischain.com",
            chainId: 100,

            accounts: [process.env.PRIVATE_KEY]
        },
        fuse: {
            url: "https://rpc.fuse.io",
            chainId: 122,

            accounts: [process.env.PRIVATE_KEY]
        },
        shardeum: {
            url: "https://liberty10.shardeum.org",
            chainId: 8080,

            accounts: [process.env.PRIVATE_KEY]
        },
        quai: {
            url: "https://quai.node.endpoint.io",
            chainId: 994,

            accounts: [process.env.PRIVATE_KEY]
        },
        arbitrumNova: {
            url: "https://nova.arbitrum.io/rpc",
            chainId: 42170,

            accounts: [process.env.PRIVATE_KEY]
        },
        zkSync: {
            url: "https://zksync2-testnet.zksync.dev",
            chainId: 280,

            accounts: [process.env.PRIVATE_KEY]
        },
        scroll: {
            url: "https://prealpha-rpc.scroll.io/l1",
            chainId: 534351,

            accounts: [process.env.PRIVATE_KEY]
        },
        scrollL2: {
            url: "https://prealpha-rpc.scroll.io/l2",
            chainId: 534354,

            accounts: [process.env.PRIVATE_KEY]
        },
        oasisEmerald: {
            url: "https://emerald.oasis.dev",
            chainId: 42262,

            accounts: [process.env.PRIVATE_KEY]
        },
        oasisSapphire: {
            url: "https://sapphire.oasis.io",
            chainId: 23294,

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
    solidity: {
        version: "0.8.17",
        docker: false,
        settings: {
            optimizer: {
                enabled: true,
                runs: 99999,
            },
        },
    },
    mocha: {
        timeout: 20000
    },
    etherscan: {
        // Your API key for Etherscan
        // Obtain one at https://etherscan.io/
        apiKey: {
            mainnet: process.env.ETHSCAN_API_KEY,
            avalanche: process.env.AVALANCHE_API_KEY,
            polygon: process.env.POLYGON_API_KEY,
            moonbeam: process.env.MOONBEEN_API_KEY,
            arbitrumOne: process.env.ARBITRUM_ONE_API_KEY,
            gnosis: process.env.GNOSIS_API_KEY,
            rinkeby: process.env.ETHSCAN_API_KEY,
            bsc: process.env.BSCSCAN_API_KEY,
            bscTestnet: process.env.BSCSCAN_API_KEY,
            godwoken: process.env.GODWOKEN_API_KEY,
            oasisEmerald: process.env.OASIS_API_KEY,
            oasisSapphire: process.env.OASIS_API_KEY

            // polygonMumbai: secrets.api_key_polygon,
        },
        customChains: [
            {
                network: "godwoken",
                chainId: 71402,
                urls: {
                    apiURL: "https://gw-mainnet-explorer.nervosdao.community/api",
                    browserURL: "https://gw-mainnet-explorer.nervosdao.community"
                }
            },
            {
                network: "oasisEmerald",
                chainId: 42262,
                urls: {
                    apiURL: "https://explorer.emerald.oasis.dev/api",
                    browserURL: "https://explorer.emerald.oasis.dev"
                }
            },
            {
                network: "oasisSapphire",
                chainId: 23294,
                urls: {
                    apiURL: "https://explorer.sapphire.oasis.io/api",
                    browserURL: "https://explorer.sapphire.oasis.io"
                }
            }
        ]
    },
};
