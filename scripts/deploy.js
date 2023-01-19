const hre = require("hardhat");
const {ethers, upgrades} = require("hardhat");

async function main() {

    async function getFactories() {
        let factories = {};

        factories.MultiSend = await ethers.getContractFactory(
            "MultiSend"
        );
        return factories;
    }

    const owner = await ethers.getSigners();

    const contracts = {};
    contracts.factories = await getFactories();

    contracts.MultiSend = await upgrades.deployProxy(
        contracts.factories.MultiSend,
        {
            initializer: "initialize",
            kind: "uups",
        }
    );

    await contracts.MultiSend.deployed();

    console.log(`multiSend deployed to ${contracts.address}`);

    setTimeout(() => {
        hre.run("verify:verify", {
            address: contracts.address,
            arguments: owner
        });
    }, 5000)

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
