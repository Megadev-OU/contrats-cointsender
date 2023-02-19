const {hre,ethers, upgrades} = require("hardhat");
const {loadFixture} = require("@nomicfoundation/hardhat-network-helpers");


async function main2() {

    async function getFactories(owner) {
        let factories = {};

        factories.MultiSend = await ethers.getContractFactory(
            "MultiSendV1_1",
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

    return {contract, MultiSend, owner, addr1, addr2};
}

async function main() {
    const oldContract = "";
    const {contract} = await loadFixture(main2);
    const newContractFactory = await ethers.getContractFactory("MultiSendV1_1");
  
    multisend = await upgrades.upgradeProxy(
        contract.address,
        newContractFactory
    );
    console.log(multisend);

}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
