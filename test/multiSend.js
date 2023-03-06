const { ethers, upgrades } = require('hardhat');
const { expect } = require('chai');
const { BigNumber } = require('ethers');
const hre = require('hardhat');
const { loadFixture } = require('@nomicfoundation/hardhat-network-helpers');

async function main() {
  async function getFactories(owner) {
    let factories = {};

    factories.MultiSendV1_0 = await ethers.getContractFactory('MultiSendV1_0', owner);

    factories.MultiSendV1_1 = await ethers.getContractFactory('MultiSendV1_1', owner);

    factories.MultiSend = await ethers.getContractFactory('MultiSend', owner);

    factories.ERC20Token = await ethers.getContractFactory('ERC20Mock');

    return factories;
  }

  const [owner] = await ethers.getSigners();

  const addr1 = '0x1234567890123456789012345678901234567890';
  const addr2 = '0x2234567890123456789012345678901234567890';

  const contracts = {};
  contracts.factories = await getFactories(owner);

  const MultiSendV1_0 = (contracts.MultiSendV1_0 = await upgrades.deployProxy(
    contracts.factories.MultiSendV1_0,
    {
      initializer: 'initialize',
      kind: 'uups',
    },
  ));

  await contracts.MultiSendV1_0.deployed();

  const MultiSendV1_1 = (contracts.MultiSendV1_1 = await upgrades.deployProxy(
    contracts.factories.MultiSendV1_1,
    {
      initializer: 'initialize',
      kind: 'uups',
    },
  ));

  await contracts.MultiSendV1_1.deployed();

  const MultiSend = (contracts.MultiSendV1_2 = await upgrades.deployProxy(
    contracts.factories.MultiSend,
    {
      initializer: 'initialize',
      kind: 'uups',
    },
  ));

  const contract = await contracts.MultiSendV1_2.deployed();

  contracts.ERC20Token = await contracts.factories.ERC20Token.deploy(
    'Test Token',
    'TEST',
    ethers.utils.parseEther('1000'),
  );

  const token = await contracts.ERC20Token.deployed();

  return { contract, MultiSend, owner, addr1, addr2, token };
}

describe('multiSendDiffEth', function () {
  const percent = 10;
  const recipient1Amount = ethers.utils.parseEther('1');
  const recipient2Amount = ethers.utils.parseEther('1');
  const totalAmount = recipient1Amount.add(recipient2Amount);
  const taxAmount = totalAmount.mul(percent).div(10000);
  const valueToSend = totalAmount.add(taxAmount);

  it('should send ether to multiple recipients and transfer taxes to bank', async function () {
    const { contract, addr1, addr2 } = await loadFixture(main);

    const bank = await contract.bank();

    await contract.multiSendDiffEth([addr1, addr2], [recipient1Amount, recipient2Amount], {
      value: valueToSend,
    });

    expect(await ethers.provider.getBalance(addr1)).to.equal(recipient1Amount);
    expect(await ethers.provider.getBalance(addr2)).to.equal(recipient2Amount);
    expect(await ethers.provider.getBalance(bank)).to.equal(taxAmount);
  });

  it('should revert if there are no recipients', async function () {
    const { contract } = await loadFixture(main);

    await expect(contract.multiSendDiffEth([], [])).to.be.reverted;
  });

  it("should revert if the number of recipients and amounts don't match", async function () {
    const { contract, addr1, addr2 } = await loadFixture(main);

    await expect(contract.multiSendDiffEth([addr1, addr2], [recipient1Amount])).to.be.reverted;
  });

  it('should revert if the total amount sent is greater than the value sent', async function () {
    const { contract, addr1, addr2 } = await loadFixture(main);

    await expect(
      contract.multiSendDiffEth([addr1, addr2], [valueToSend, recipient1Amount]),
    ).to.be.revertedWith('Low balance');
  });

  it('should revert if a recipient is the zero address', async function () {
    const { contract, addr2 } = await loadFixture(main);

    await expect(
      contract.multiSendDiffEth(
        [ethers.constants.AddressZero, addr2],
        [recipient1Amount, recipient2Amount],
        { value: valueToSend },
      ),
    ).to.be.revertedWith('Recipient must be not zero address');
  });

  it('should revert if an amount is zero', async function () {
    const { contract, addr1, addr2 } = await loadFixture(main);

    await expect(
      contract.multiSendDiffEth([addr1, addr2], [recipient1Amount, 0], { value: valueToSend }),
    ).to.be.reverted;
  });
});

describe('multiSendDiffToken', () => {
  it('should revert if recipient address is zero address', async function () {
    const { contract, addr1, token } = await loadFixture(main);

    const recipients = [addr1, ethers.constants.AddressZero];
    const amounts = [ethers.utils.parseEther('10'), ethers.utils.parseEther('20')];

    await token.approve(contract.address, ethers.utils.parseEther('30'));

    await expect(
      contract.multiSendDiffToken(recipients, amounts, token.address),
    ).to.be.revertedWith('Recipient must be not zero address');
  });

  it('reverts when recipients array is empty', async () => {
    const { contract, token } = await loadFixture(main);

    await expect(contract.multiSendDiffToken([], [], token.address)).to.be.reverted;
  });

  it('reverts when recipients array length is not equal to amounts array length', async () => {
    const { contract, addr1, token } = await loadFixture(main);

    await expect(contract.multiSendDiffToken([addr1], [], token.address)).to.be.reverted;
  });

  it('reverts when any amount is 0', async () => {
    const { contract, addr1, addr2, token } = await loadFixture(main);

    await token.approve(contract.address, ethers.utils.parseEther('200'));

    await expect(
      contract.multiSendDiffToken([addr1, addr2], [100, 0], token.address),
    ).to.be.revertedWith('Value must be more than 0');
  });

  it('reverts when sender does not have enough balance', async () => {
    const { contract, addr1, addr2, token, owner } = await loadFixture(main);

    await token.approve(contract.address, ethers.utils.parseEther('200'));
    const balance = await token.balanceOf(owner.address);

    await expect(
      contract.multiSendDiffToken(
        [addr1, addr2],
        [balance.toString(), balance.toString()],
        token.address,
      ),
    ).to.be.revertedWith('Influence balance');
  });

  it('reverts when sender has not approved enough tokens for the contract', async () => {
    const { contract, addr1, addr2, owner, token } = await loadFixture(main);

    await token.transfer(owner.address, 1000);
    await token.connect(owner).approve(contract.address, 50);

    await expect(
      contract.multiSendDiffToken([addr1, addr2], [100, 100], token.address),
    ).to.be.revertedWith('Influence allowance');
  });

  it('should transfer tokens to recipients and bank', async function () {
    const { contract, addr1, addr2, token } = await loadFixture(main);

    const amount1 = ethers.utils.parseEther('1');
    const amount2 = ethers.utils.parseEther('2');
    const totalAmount = amount1.add(amount2);
    const percent = 10;
    const taxAmount = totalAmount.mul(percent).div(10000);
    const bank = await contract.bank();

    await token.approve(contract.address, ethers.utils.parseEther('200'));

    await contract.multiSendDiffToken([addr1, addr2], [amount1, amount2], token.address);

    expect(await token.balanceOf(addr1)).to.equal(amount1);
    expect(await token.balanceOf(addr2)).to.equal(amount2);
    expect(await token.balanceOf(bank)).to.equal(taxAmount);
  });
});

describe('changeBankAddress', function () {
  it('should not allow non-owners to change the bank address', async function () {
    const { contract, addr1 } = await loadFixture(main);
    const nonOwner = ethers.provider.getSigner(1);
    await expect(contract.connect(nonOwner).changeBankAddress(addr1)).to.be.rejectedWith(
      'Ownable: caller is not the owner',
    );
    expect(await contract.bank()).to.equal('0x3Ff0Dc6514d719152692188bD6F0771ADe370852');
  });

  it('should allow the owner to change the bank address', async function () {
    const { contract, addr1 } = await loadFixture(main);
    await expect(contract.changeBankAddress(addr1)).to.be.fulfilled;
    expect(await contract.bank()).to.equal(addr1);
  });
});

describe('changePercentage', async function () {
  it('should allow the owner to change the percentage', async function () {
    const { contract } = await loadFixture(main);

    await expect(contract.changePercentage(20)).to.be.fulfilled;
    expect(await contract.percent()).to.equal(20);
  });

  it('should not allow non-owners to change the percentage', async function () {
    const { contract } = await loadFixture(main);
    const nonOwner = ethers.provider.getSigner(1);
    await expect(contract.connect(nonOwner).changePercentage(20)).to.be.rejectedWith(
      'Ownable: caller is not the owner',
    );
    expect(await contract.percent()).to.equal(10);
  });
});

describe('Check upgrade', function () {
  it('upgrade contract v1.0 to v1.1 check', async () => {
    const contracts = await loadFixture(main);
    const contractV1_1 = await ethers.getContractFactory('MultiSendV1_1');
    const contractV1_2 = await ethers.getContractFactory('MultiSend');

    multisend = await upgrades.upgradeProxy(contracts.MultiSend.address, contractV1_1);

    await multisend.deployed();

    multisend = await upgrades.upgradeProxy(multisend.address, contractV1_2);

    await multisend.deployed();
  });
});
