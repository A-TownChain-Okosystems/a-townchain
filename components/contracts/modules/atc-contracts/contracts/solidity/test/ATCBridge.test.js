// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
const { expect } = require("chai");
const { ethers }  = require("hardhat");
const { time }    = require("@nomicfoundation/hardhat-network-helpers");

/**
 * ATCBridge.test.js — Cross-Chain Bridge (ATC-5000)
 * Tests: Lock, Release, Timelock, Relayer, Rate-Limiting, Pause
 */
describe("ATCBridge (ATC-5000)", function () {

  let bridge, atcToken;
  let owner, user1, user2;
  let relayer1, relayer2, relayer3, relayer4, relayer5;

  const INITIAL_ATC   = ethers.parseEther("10000000");
  const LOCK_AMOUNT   = ethers.parseEther("1000");
  const LARGE_AMOUNT  = ethers.parseEther("200000");   // > 100k → Timelock
  const MAX_AMOUNT    = ethers.parseEther("1000000");
  const OVER_MAX      = ethers.parseEther("1000001");
  const DEST_CHAIN    = "ethereum";
  const DEST_ADDRESS  = "0x1234567890123456789012345678901234567890";

  beforeEach(async function () {
    [owner, user1, user2, relayer1, relayer2, relayer3, relayer4, relayer5] =
      await ethers.getSigners();

    // ATCToken
    const ATCToken = await ethers.getContractFactory("ATCToken");
    atcToken = await ATCToken.deploy();
    await atcToken.waitForDeployment();
    await atcToken.registerMiner(owner.address);
    await atcToken.connect(owner).mint(owner.address, INITIAL_ATC);
    await atcToken.transfer(user1.address, ethers.parseEther("500000"));
    await atcToken.transfer(user2.address, ethers.parseEther("500000"));

    // Bridge deployen
    const Bridge = await ethers.getContractFactory("ATCBridge");
    bridge = await Bridge.deploy(
      await atcToken.getAddress(),
      [relayer1.address, relayer2.address, relayer3.address,
       relayer4.address, relayer5.address]
    );
    await bridge.waitForDeployment();

    // User1 gibt Bridge Erlaubnis für ATC
    await atcToken.connect(user1).approve(
      await bridge.getAddress(), ethers.parseEther("2000000")
    );
    await atcToken.connect(user2).approve(
      await bridge.getAddress(), ethers.parseEther("2000000")
    );
  });

  // ── 1. Deployment ─────────────────────────────────────
  describe("Deployment", function () {
    it("sollte Relayer korrekt registrieren", async function () {
      expect(await bridge.isRelayer(relayer1.address)).to.be.true;
      expect(await bridge.isRelayer(relayer2.address)).to.be.true;
      expect(await bridge.isRelayer(user1.address)).to.be.false;
      expect(await bridge.relayerCount()).to.equal(5n);
    });

    it("sollte Konstanten korrekt setzen", async function () {
      expect(await bridge.MAX_TX_AMOUNT()).to.equal(MAX_AMOUNT);
      expect(await bridge.RELAYER_THRESHOLD()).to.equal(3n);
    });

    it("sollte nur mit exakt 5 Relayern deploybar sein", async function () {
      const Bridge2 = await ethers.getContractFactory("ATCBridge");
      await expect(
        Bridge2.deploy(await atcToken.getAddress(),
          [relayer1.address, relayer2.address]) // nur 2
      ).to.be.revertedWith("Need exactly 5 relayers");
    });
  });

  // ── 2. Lock ATC ───────────────────────────────────────
  describe("lockATC", function () {
    it("sollte ATC korrekt sperren", async function () {
      const balBefore = await atcToken.balanceOf(user1.address);
      const tx = await bridge.connect(user1).lockATC(
        LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS
      );
      await tx.wait();

      const balAfter = await atcToken.balanceOf(user1.address);
      expect(balBefore - balAfter).to.equal(LOCK_AMOUNT);

      const bridgeBal = await atcToken.balanceOf(await bridge.getAddress());
      expect(bridgeBal).to.equal(LOCK_AMOUNT);
    });

    it("sollte ATCLocked-Event emittieren", async function () {
      await expect(
        bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS)
      ).to.emit(bridge, "ATCLocked")
        .withArgs(0n, user1.address, LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
    });

    it("sollte BridgeTx korrekt speichern", async function () {
      await bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
      const tx_ = await bridge.getBridgeTx(0);
      expect(tx_.sender).to.equal(user1.address);
      expect(tx_.amount).to.equal(LOCK_AMOUNT);
      expect(tx_.destinationChain).to.equal(DEST_CHAIN);
      expect(tx_.status).to.equal(1n); // Locked
    });

    it("sollte bei zu großem Betrag revertieren", async function () {
      await expect(
        bridge.connect(user1).lockATC(OVER_MAX, DEST_CHAIN, DEST_ADDRESS)
      ).to.be.revertedWith("Invalid amount");
    });

    it("sollte Daily-Limit tracken", async function () {
      await bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
      expect(await bridge.dailyVolume()).to.equal(LOCK_AMOUNT);
    });
  });

  // ── 3. Timelock für große TX ───────────────────────────
  describe("Timelock (> 100.000 ATC)", function () {
    it("sollte executeAfter = jetzt + 24h setzen für große TX", async function () {
      const before = await time.latest();
      await bridge.connect(user1).lockATC(LARGE_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
      const tx_ = await bridge.getBridgeTx(0);
      expect(tx_.executeAfter).to.be.gte(before + 24 * 3600 - 5);
    });

    it("sollte executeAfter = jetzt für kleine TX setzen", async function () {
      const before = await time.latest();
      await bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
      const tx_ = await bridge.getBridgeTx(0);
      expect(tx_.executeAfter).to.be.lte(before + 10);
    });
  });

  // ── 4. Relayer: Sign + Execute ─────────────────────────
  describe("Relayer Signatur & Release", function () {
    const CROSS_HASH = ethers.keccak256(ethers.toUtf8Bytes("cross-chain-tx-001"));

    beforeEach(async function () {
      await bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
    });

    it("sollte Relayer-Signatur akzeptieren", async function () {
      await expect(
        bridge.connect(relayer1).signRelease(0)
      ).to.emit(bridge, "RelayerSigned")
        .withArgs(0n, relayer1.address, 1n);

      expect(await bridge.signatureCount(0)).to.equal(1n);
    });

    it("sollte doppelte Signatur ablehnen", async function () {
      await bridge.connect(relayer1).signRelease(0);
      await expect(
        bridge.connect(relayer1).signRelease(0)
      ).to.be.revertedWith("Already signed");
    });

    it("sollte bei 3 Signaturen Release erlauben", async function () {
      await bridge.connect(relayer1).signRelease(0);
      await bridge.connect(relayer2).signRelease(0);
      await bridge.connect(relayer3).signRelease(0);

      const user2BalBefore = await atcToken.balanceOf(user2.address);
      await bridge.connect(relayer1).executeRelease(0, user2.address, CROSS_HASH);
      const user2BalAfter  = await atcToken.balanceOf(user2.address);

      expect(user2BalAfter - user2BalBefore).to.equal(LOCK_AMOUNT);
    });

    it("sollte ATCReleased-Event emittieren", async function () {
      await bridge.connect(relayer1).signRelease(0);
      await bridge.connect(relayer2).signRelease(0);
      await bridge.connect(relayer3).signRelease(0);

      await expect(
        bridge.connect(relayer1).executeRelease(0, user2.address, CROSS_HASH)
      ).to.emit(bridge, "ATCReleased")
        .withArgs(0n, user2.address, LOCK_AMOUNT, CROSS_HASH);
    });

    it("sollte bei < 3 Signaturen revertieren", async function () {
      await bridge.connect(relayer1).signRelease(0);
      await bridge.connect(relayer2).signRelease(0);
      await expect(
        bridge.connect(relayer1).executeRelease(0, user2.address, CROSS_HASH)
      ).to.be.revertedWith("Not enough signatures");
    });

    it("sollte Replay-Angriff via processedHash verhindern", async function () {
      await bridge.connect(relayer1).signRelease(0);
      await bridge.connect(relayer2).signRelease(0);
      await bridge.connect(relayer3).signRelease(0);
      await bridge.connect(relayer1).executeRelease(0, user2.address, CROSS_HASH);

      // Zweite Lock-TX
      await bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
      await bridge.connect(relayer1).signRelease(1);
      await bridge.connect(relayer2).signRelease(1);
      await bridge.connect(relayer3).signRelease(1);

      // Gleicher Hash → Replay-Schutz
      await expect(
        bridge.connect(relayer1).executeRelease(1, user2.address, CROSS_HASH)
      ).to.be.revertedWith("Already processed");
    });

    it("sollte Nicht-Relayer abweisen", async function () {
      await expect(
        bridge.connect(user1).signRelease(0)
      ).to.be.revertedWith("Not a relayer");
    });
  });

  // ── 5. Emergency Cancel ───────────────────────────────
  describe("Emergency Cancel", function () {
    it("sollte TX canceln und ATC zurückgeben", async function () {
      await bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS);
      const balBefore = await atcToken.balanceOf(user1.address);
      await bridge.connect(owner).cancelBridgeTx(0);
      const balAfter = await atcToken.balanceOf(user1.address);
      expect(balAfter - balBefore).to.equal(LOCK_AMOUNT);
      const tx_ = await bridge.getBridgeTx(0);
      expect(tx_.status).to.equal(4n); // Cancelled
    });
  });

  // ── 6. Pause ──────────────────────────────────────────
  describe("Emergency Pause", function () {
    it("sollte Bridge pausieren und lockATC sperren", async function () {
      await bridge.connect(owner).emergencyPause();
      await expect(
        bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS)
      ).to.be.reverted;
    });

    it("sollte nach Unpause wieder funktionieren", async function () {
      await bridge.connect(owner).emergencyPause();
      await bridge.connect(owner).unpause();
      await expect(
        bridge.connect(user1).lockATC(LOCK_AMOUNT, DEST_CHAIN, DEST_ADDRESS)
      ).to.not.be.reverted;
    });
  });

  // ── 7. Relayer-Management ─────────────────────────────
  describe("Relayer Management", function () {
    it("sollte neuen Relayer hinzufügen", async function () {
      await bridge.connect(owner).addRelayer(user1.address);
      expect(await bridge.isRelayer(user1.address)).to.be.true;
      expect(await bridge.relayerCount()).to.equal(6n);
    });

    it("sollte Relayer entfernen wenn > Threshold", async function () {
      await bridge.connect(owner).addRelayer(user1.address);
      await bridge.connect(owner).removeRelayer(user1.address);
      expect(await bridge.isRelayer(user1.address)).to.be.false;
    });

    it("sollte Entfernung ablehnen wenn zu wenige Relayer", async function () {
      // Genau 5 Relayer, Threshold = 3 → darf entfernen bis auf 3
      await bridge.connect(owner).removeRelayer(relayer4.address);
      await bridge.connect(owner).removeRelayer(relayer5.address);
      // Jetzt 3 → darf nicht weiter entfernen
      await expect(
        bridge.connect(owner).removeRelayer(relayer3.address)
      ).to.be.revertedWith("Too few relayers");
    });
  });
});
