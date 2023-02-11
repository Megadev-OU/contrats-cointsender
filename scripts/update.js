const hre = require("hardhat");
const {ethers, upgrades} = require("hardhat");

async function main() {
    const {oldContract} = "";
    const newContract = "";

    multisend = await upgrades.upgradeProxy(
        oldContract,
        newContract
    );
    console.log(multisend);

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
