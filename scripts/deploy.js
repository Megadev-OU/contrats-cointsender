const hre = require("hardhat");
const {ethers, upgrades} = require("hardhat");

async function main() {

    async function getFactories(owner) {
        let factories = {};

        factories.MultiSend = await ethers.getContractFactory(
            "MultiSend",
            owner
        );
        return factories;
    }

    [owner] = await ethers.getSigners();

    const contracts = {};
    contracts.factories = await getFactories(owner);

    contracts.MultiSend = await upgrades.deployProxy(
        contracts.factories.MultiSend,
        {
            initializer: "initialize",
            kind: "uups",
        }
    );

    await contracts.MultiSend.deployed();

    console.log(`multiSend deployed to ${contracts.MultiSend.address}`);

    // setTimeout(() => {
    //     hre.run("verify:verify", {
    //         address: contracts.MultiSend.address,
    //         arguments: owner
    //     });
    // }, 5000)

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
