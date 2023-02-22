const { hre, ethers, upgrades } = require("hardhat");
const { loadFixture } = require("@nomicfoundation/hardhat-network-helpers");


async function main2() {

    async function getFactories(owner) {
        let factories = {};

        factories.MultiSend = await ethers.getContractFactory(
            "MultiSend",
            owner
        );
        return factories;
    }

    const [owner, addr1, addr2] = await ethers.getSigners();

    const contracts = {};
    contracts.factories = await getFactories(owner);

    const MultiSend = contracts.MultiSend = await upgrades.deployProxy(
        contracts.factories.MultiSend,
        {
            initializer: "initialize",
            kind: "uups",
        }
    );

    const contract = await contracts.MultiSend.deployed();

    return { contract, MultiSend, owner, addr1, addr2 };
}

async function main() {
    const oldContract = "0x1bb79e75a062ff90F8E79FE281f41324C3052afc";
    const newContractAddress = "0x9B793EbE7353D2afcfa8f8310247aB4AF437cf96";
    // const newContractFactoryV1_1 = await ethers.getContractFactory("MultiSendV1_1").forceImport(oldContract);
    const newContractFactory = await ethers.getContractFactory("MultiSend");
    const newContract = await upgrades.forceImport(newContractAddress, 
        // newContractFactory, 
        {
        kind: "uups",
    });
    // val old = newContractFactoryV1_1.attach(oldContract)
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
