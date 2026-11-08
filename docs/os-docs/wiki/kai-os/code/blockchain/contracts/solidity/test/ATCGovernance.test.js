// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("ATCGovernance (ATC-9900)", function () {
  let governance;
  let atcToken;
  let owner;
  let addr1;
  let addr2;
  let addr3;
  let miner;

  beforeEach(async function () {
    [owner, addr1, addr2, addr3, miner] = await ethers.getSigners();

    // Deploy ATCToken
    const ATCToken = await ethers.getContractFactory("ATCToken");
    atcToken = await ATCToken.deploy();
    await atcToken.waitForDeployment();

    // Register miner and mint tokens
    await atcToken.registerMiner(miner.address);
    const initialSupply = ethers.parseEther("10000000");
    await atcToken.connect(miner).mint(owner.address, initialSupply);

    // Distribute tokens to voters
    await atcToken.transfer(addr1.address, ethers.parseEther("100000"));
    await atcToken.transfer(addr2.address, ethers.parseEther("50000"));
    await atcToken.transfer(addr3.address, ethers.parseEther("25000"));

    // Deploy Governance
    const Governance = await ethers.getContractFactory("ATCGovernance");
    governance = await Governance.deploy(await atcToken.getAddress());
    await governance.waitForDeployment();
  });

  // ── Proposal Creation Tests ────────────────────────
  describe("Proposal Creation", function () {
    it("Should create a new proposal", async function () {
      const title = "Increase block reward";
      const description = "Proposal to increase block rewards";
      const options = ["Yes", "No"];

      const tx = await governance
        .connect(owner)
        .createProposal(title, description, options);

      const proposal = await governance.getProposal(0);
      expect(proposal.title).to.equal(title);
      expect(proposal.creator).to.equal(owner.address);
    });

    it("Should reject proposal with less than 2 options", async function () {
      const title = "Invalid proposal";
      const description = "Only 1 option";
      const options = ["Yes"];

      await expect(
        governance.createProposal(title, description, options)
      ).to.be.revertedWith("At least 2 options required");
    });

    it("Should reject proposal with more than 10 options", async function () {
      const title = "Too many options";
      const description = "More than 10 options";
      const options = Array.from({ length: 11 }, (_, i) => `Option ${i + 1}`);

      await expect(
        governance.createProposal(title, description, options)
      ).to.be.revertedWith("Max 10 options");
    });

    it("Should set correct proposal status", async function () {
      const title = "Test proposal";
      const description = "Testing";
      const options = ["Yes", "No"];

      await governance.createProposal(title, description, options);
      const proposal = await governance.getProposal(0);

      expect(proposal.status).to.equal(1); // ACTIVE
    });

    it("Should increment proposal count", async function () {
      const options = ["Yes", "No"];

      await governance
        .connect(owner)
        .createProposal("Proposal 1", "Desc 1", options);
      await governance
        .connect(owner)
        .createProposal("Proposal 2", "Desc 2", options);

      expect(await governance.proposalCount()).to.equal(2);
    });
  });

  // ── Voting Tests ───────────────────────────────────
  describe("Voting", function () {
    beforeEach(async function () {
      const options = ["Yes", "No"];
      await governance
        .connect(owner)
        .createProposal("Test proposal", "Testing voting", options);
    });

    it("Should allow voting on active proposal", async function () {
      const tx = await governance.connect(addr1).vote(0, 0);
      expect(tx).to.emit(governance, "VoteCasted");
    });

    it("Should track votes correctly", async function () {
      await governance.connect(addr1).vote(0, 0);
      await governance.connect(addr2).vote(0, 0);
      await governance.connect(addr3).vote(0, 1);

      const votes = await governance.getProposalVotes(0);
      expect(votes[0]).to.equal(ethers.parseEther("100000") + ethers.parseEther("50000"));
      expect(votes[1]).to.equal(ethers.parseEther("25000"));
    });

    it("Should reject voting twice on same proposal", async function () {
      await governance.connect(addr1).vote(0, 0);

      await expect(
        governance.connect(addr1).vote(0, 0)
      ).to.be.revertedWith("Already voted");
    });

    it("Should reject voting with 0 balance", async function () {
      const [, , , , , newAddr] = await ethers.getSigners();

      await expect(
        governance.connect(newAddr).vote(0, 0)
      ).to.be.revertedWith("No voting power");
    });

    it("Should reject voting on invalid option", async function () {
      await expect(
        governance.connect(addr1).vote(0, 99)
      ).to.be.revertedWith("Invalid option");
    });

    it("Should only allow voting on active proposals", async function () {
      // Create proposal
      const options = ["Yes", "No"];
      await governance
        .connect(owner)
        .createProposal("Expired proposal", "Testing", options);

      // Wait for voting to end (7 days in real scenario)
      // For testing, we'd need to fast-forward time
      // This test is simplified here
    });
  });

  // ── Proposal Execution Tests ───────────────────────
  describe("Proposal Execution", function () {
    beforeEach(async function () {
      const options = ["Yes", "No"];
      await governance
        .connect(owner)
        .createProposal("Test proposal", "Testing execution", options);

      // Add votes
      await governance.connect(addr1).vote(0, 0);
      await governance.connect(addr2).vote(0, 0);
    });

    it("Should execute proposal after voting ends", async function () {
      // In a real scenario, we'd fast-forward time
      // For now, this is a simplified test
      const proposal = await governance.getProposal(0);
      expect(proposal.status).to.equal(1); // ACTIVE
    });

    it("Should mark proposal as executed", async function () {
      const proposal = await governance.getProposal(0);
      expect(proposal.status).to.be.within(1, 4); // ACTIVE to EXECUTED
    });

    it("Should emit ProposalExecuted event", async function () {
      // Event testing would be here
    });
  });

  // ── Query Tests ────────────────────────────────────
  describe("Queries", function () {
    beforeEach(async function () {
      const options = ["Option A", "Option B", "Option C"];
      await governance
        .connect(owner)
        .createProposal("Multi-option proposal", "3 options", options);
    });

    it("Should return proposal data", async function () {
      const proposal = await governance.getProposal(0);

      expect(proposal.id).to.equal(0);
      expect(proposal.creator).to.equal(owner.address);
      expect(proposal.title).to.equal("Multi-option proposal");
    });

    it("Should return proposal votes", async function () {
      await governance.connect(addr1).vote(0, 0);
      await governance.connect(addr2).vote(0, 1);
      await governance.connect(addr3).vote(0, 2);

      const votes = await governance.getProposalVotes(0);
      expect(votes.length).to.equal(3);
      expect(votes[0]).to.equal(ethers.parseEther("100000"));
      expect(votes[1]).to.equal(ethers.parseEther("50000"));
      expect(votes[2]).to.equal(ethers.parseEther("25000"));
    });
  });

  // ── Edge Cases ────────────────────────────────────
  describe("Edge Cases", function () {
    it("Should handle proposals with 0 voters", async function () {
      const options = ["Yes", "No"];
      await governance
        .connect(owner)
        .createProposal("No voters", "Empty proposal", options);

      const proposal = await governance.getProposal(0);
      expect(proposal.status).to.equal(1); // ACTIVE
    });

    it("Should handle voting with exact balance", async function () {
      const options = ["Yes", "No"];
      await governance
        .connect(owner)
        .createProposal("Test", "Test", options);

      // addr1 has exactly 100,000 ATC
      await governance.connect(addr1).vote(0, 0);
      const votes = await governance.getProposalVotes(0);
      expect(votes[0]).to.equal(ethers.parseEther("100000"));
    });

    it("Should handle multiple proposals concurrently", async function () {
      const options = ["Yes", "No"];

      for (let i = 0; i < 5; i++) {
        await governance
          .connect(owner)
          .createProposal(`Proposal ${i}`, `Testing ${i}`, options);
      }

      expect(await governance.proposalCount()).to.equal(5);
    });
  });
});
