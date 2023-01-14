const hre = require("hardhat");

async function main() {
    const ownerAddress = '0xe9D3F501B082Ba426b4Fb1be6b00be64D486d4d9'
    const MultiSend = await hre.ethers.getContractFactory("MultiSend");
    const multiSend = await MultiSend.deploy(ownerAddress);
    await multiSend.deployed();

    console.log(`multiSend deployed to ${multiSend.address}`);

    setTimeout(() => {
        hre.run("verify:verify", {
            address: multiSend.address,
            constructorArguments: [ownerAddress]
        });
    }, 5000)

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
