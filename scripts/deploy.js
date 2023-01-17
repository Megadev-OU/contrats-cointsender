const hre = require("hardhat");

async function main() {
    const MultiSend = await hre.ethers.getContractFactory("MultiSend");
    const multiSend = await MultiSend.deploy();
    await multiSend.deployed();

    console.log(`multiSend deployed to ${multiSend.address}`);

    setTimeout(() => {
        hre.run("verify:verify", {
            address: multiSend.address
        });
    }, 5000)

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
