

const {ethers, upgrades} = require("hardhat");


async function main() {
    const [owner] = await ethers.getSigners();
    const contractAddress = "0x1bb79e75a062ff90F8E79FE281f41324C3052afc";
    const multiSendFactory = await ethers.getContractFactory(
        "MultiSend",
        owner
    );
    const constract = await upgrades.forceImport(contractAddress, 
        multiSendFactory, 
        {
        kind: "uups",
    });


  const tx = await constract.createSecret(
    'ingredient',
    30 /* seconds */,
    Buffer.from('brussels sprouts'),
  );
  console.log('Storing a secret in', tx.hash);
  await tx.wait();

    // console.log('Checking the secret');
    // await constract.connect(ethers.provider).callStatic.revealSecret(0);
    // console.log('Uh oh. The secret was available!');
    // process.exit(1);

  console.log('Waiting...');
  await new Promise((resolve) => setTimeout(resolve, 30_000));
  console.log('Checking the secret again');
  await (await constract.revealSecret(0)).wait(); // Reveal the secret.
  const secret = await constract.callStatic.revealSecret(0); // Get the value.
  console.log('The secret ingredient is', Buffer.from(secret.slice(2), 'hex').toString());
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});