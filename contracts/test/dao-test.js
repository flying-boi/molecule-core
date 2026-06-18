const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("SimpleDAO", function () {
  let dao;
  let owner, addr1;

  beforeEach(async function () {
    [owner, addr1] = await ethers.getSigners();
    const SimpleDAO = await ethers.getContractFactory("SimpleDAO");
    dao = await SimpleDAO.deploy();
    await dao.deployed();
  });

  it("sets voting power and counts votes", async function () {
    await dao.setVotingPower(addr1.address, 10);
    await dao.newProposal("Test proposal", 3600);

    const prop = await dao.proposals(0);
    expect(prop[3].toNumber()).to.be.greaterThan(0);

    await dao.connect(addr1).vote(0, true);

    const updated = await dao.proposals(0);
    expect(updated[1].toNumber()).to.equal(10);
  });
});
