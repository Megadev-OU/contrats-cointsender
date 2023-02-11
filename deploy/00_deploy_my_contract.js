module.exports = async ({ getNamedAccounts, deployments }) => {
  
    const {deployer} = await getNamedAccounts();
    const { deploy } = deployments;
    await deploy('MultiSendV1_1', {
        contract: "MultiSendV1_1",
        from: deployer,
        args: [],
        log: true,
      });
};
module.exports.tags = ['all'];