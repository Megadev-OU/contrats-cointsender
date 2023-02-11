const {hre,ethers, upgrades} = require("hardhat");

async function main() {
    const oldContract = "";
    const newContractFactory = await ethers.getContractFactory("MultiSendV1_1");
  
    multisend = await upgrades.upgradeProxy(
        oldContract,
        newContractFactory
    );
    console.log(multisend);

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
