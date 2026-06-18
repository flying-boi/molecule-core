const hre = require("hardhat");

async function main() {
  const [deployer, payee] = await hre.ethers.getSigners();

  const DAO = await hre.ethers.getContractFactory("SimpleDAO");
  const dao = await DAO.deploy();
  await dao.deployed();
  console.log("SimpleDAO deployed to:", dao.address);

  const Escrow = await hre.ethers.getContractFactory("MilestoneEscrow");
  const escrow = await Escrow.deploy(payee.address, { value: hre.ethers.utils.parseEther("1") });
  await escrow.deployed();
  console.log("MilestoneEscrow deployed to:", escrow.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
