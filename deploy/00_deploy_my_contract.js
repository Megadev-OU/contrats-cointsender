module.exports = async ({ getNamedAccounts, deployments }) => {
  
    const {deployer} = await getNamedAccounts();
    const { deploy } = deployments;
    await deploy('MultiSend', {
        contract: "MultiSend",
        from: deployer,
        args: [],
        log: true,
      });
};
module.exports.tags = ['all'];