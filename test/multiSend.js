const {ethers, upgrades} = require('hardhat');
const {expect} = require('chai');
const {BigNumber} = require("ethers");
const hre = require("hardhat");
const {loadFixture} = require("@nomicfoundation/hardhat-network-helpers");


async function main() {

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


describe("Read Methods Test", function () {

    it("Check percent", async () => {
        const {contract} = await loadFixture(main);
        expect(await contract.percent()).to.equal(10);
    })

    it("Check bank", async () => {
        const {contract} = await loadFixture(main);
        await expect(await contract.bank()).to.equal('0x3Ff0Dc6514d719152692188bD6F0771ADe370852');
    })
})

describe("Change params", function () {

    it("Change percent", async () => {
        const {contract} = await loadFixture(main);
        await contract.changePercentage(20);
        await expect(await contract.percent()).to.equal(20);
    })

    it("Change bank", async () => {
        const {contract, owner} = await loadFixture(main);
        await contract.changeBankAddress(owner.address);
        await expect(await contract.bank()).to.equal(owner.address);
    })

})

describe("Send Money", function () {

    it("Send ETH", async () => {
        const {contract, owner} = await loadFixture(main);
        const balanceETH = await ethers.provider.getBalance(owner.address)
        await contract.multiSendDiffEth([owner.address], [ethers.utils.formatUnits(1, "wei")], {value: ethers.utils.formatUnits(1, "wei")});
    })
    //TODO: Connect token

    it("Send TOKENS", async () => {
        const {contract, owner} = await loadFixture(main);
        const balanceETH = await ethers.provider.getBalance(owner.address)
        await contract.multiSendDiffEth([owner.address], [ethers.utils.formatUnits(1, "wei")], {value: ethers.utils.formatUnits(1, "wei")});
    })

})

describe("Check reverts", function () {

    

    it("Send ETH with different length", async () => {
        const {contract, owner} = await loadFixture(main);
        await expect(contract.multiSendDiffEth([owner.address, owner.address], [ethers.utils.formatUnits(2, "wei")], {value: ethers.utils.formatUnits(1, "wei")})).to.be.reverted;
    })

    it("Send ETH with Low balance", async () => {
        const {contract, owner} = await loadFixture(main);
        await expect(contract.multiSendDiffEth([owner.address], [ethers.utils.formatUnits(100, "wei")], {value: ethers.utils.formatUnits(20, "wei")})).to.be.reverted;
    })

    it("Send ETH with zero amount", async () => {
        const {contract, owner} = await loadFixture(main);
        await expect(contract.multiSendDiffEth([owner.address], [0], {value: ethers.utils.formatUnits(1, "wei")})).to.be.reverted;
    })

    it("Send ETH with amount more then payable", async () => {
        const {contract, owner} = await loadFixture(main);
        await expect(contract.multiSendDiffEth([owner.address], [ethers.utils.formatUnits(10000, "wei")], {value: ethers.utils.formatUnits(1, "wei")})).to.be.reverted;
    })

    it("Send ETH with empty recipients", async () => {
        const {contract} = await loadFixture(main);
        await expect(contract.multiSendDiffEth([], [ethers.utils.formatUnits(1, "wei")], {value: ethers.utils.formatUnits(1, "wei")})).to.be.reverted;
    })


    it("upgrade contract check", async () => {
        const {contract} = await loadFixture(main);
        const contract2 = await ethers.getContractFactory("MultiSendV1_1");
  
        multisend = await upgrades.upgradeProxy(
            contract.address,
            contract2
        );
    });
})