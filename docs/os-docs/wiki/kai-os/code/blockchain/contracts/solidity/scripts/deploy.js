// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
  console.log("🚀 Deploying A-TownChain Smart Contracts...\n");

  const [deployer] = await ethers.getSigners();
  console.log(`📝 Deploying contracts with account: ${deployer.address}\n`);

  // ── Deploy ATCToken ──────────────────────────────────
  console.log("1️⃣  Deploying ATCToken (ERC20)...");
  const ATCToken = await hre.ethers.getContractFactory("ATCToken");
  const atcToken = await ATCToken.deploy();
  await atcToken.waitForDeployment();
  const atcTokenAddress = await atcToken.getAddress();
  console.log(`   ✅ ATCToken deployed to: ${atcTokenAddress}\n`);

  // Register deployer as miner
  console.log("   📍 Registering deployer as miner...");
  const registerTx = await atcToken.registerMiner(deployer.address);
  await registerTx.wait();
  console.log(`   ✅ Miner registered\n`);

  // Mint initial supply
  console.log("   📍 Minting initial supply (1M ATC)...");
  const initialSupply = hre.ethers.parseEther("1000000");
  const mintTx = await atcToken.mint(deployer.address, initialSupply);
  await mintTx.wait();
  console.log(`   ✅ Initial supply minted\n`);

  // ── Deploy Shivamon ──────────────────────────────────
  console.log("2️⃣  Deploying Shivamon (ERC721)...");
  const Shivamon = await hre.ethers.getContractFactory("Shivamon");
  const shivamon = await Shivamon.deploy(atcTokenAddress);
  await shivamon.waitForDeployment();
  const shivamonAddress = await shivamon.getAddress();
  console.log(`   ✅ Shivamon deployed to: ${shivamonAddress}\n`);

  // ── Deploy ATCGovernance ──────────────────────────────
  console.log("3️⃣  Deploying ATCGovernance (DAO)...");
  const ATCGovernance = await hre.ethers.getContractFactory("ATCGovernance");
  const governance = await ATCGovernance.deploy(atcTokenAddress);
  await governance.waitForDeployment();
  const governanceAddress = await governance.getAddress();
  console.log(`   ✅ ATCGovernance deployed to: ${governanceAddress}\n`);

  // ── Save Deployment Info ──────────────────────────────
  const deploymentInfo = {
    network: hre.network.name,
    timestamp: new Date().toISOString(),
    deployer: deployer.address,
    contracts: {
      ATCToken: {
        address: atcTokenAddress,
        standard: "ATC-8300",
      },
      Shivamon: {
        address: shivamonAddress,
        standard: "ATC-9000",
      },
      ATCGovernance: {
        address: governanceAddress,
        standard: "ATC-9900",
      },
    },
  };

  // Save ABIs
  const abiDir = path.join(__dirname, "../../../config/abis");
  if (!fs.existsSync(abiDir)) {
    fs.mkdirSync(abiDir, { recursive: true });
  }

  // Save ATCToken ABI
  const atcTokenArtifact = await hre.artifacts.readArtifact("ATCToken");
  fs.writeFileSync(
    path.join(abiDir, "ATCToken.json"),
    JSON.stringify(atcTokenArtifact.abi, null, 2)
  );

  // Save Shivamon ABI
  const shivamonArtifact = await hre.artifacts.readArtifact("Shivamon");
  fs.writeFileSync(
    path.join(abiDir, "Shivamon.json"),
    JSON.stringify(shivamonArtifact.abi, null, 2)
  );

  // Save ATCGovernance ABI
  const governanceArtifact = await hre.artifacts.readArtifact("ATCGovernance");
  fs.writeFileSync(
    path.join(abiDir, "ATCGovernance.json"),
    JSON.stringify(governanceArtifact.abi, null, 2)
  );

  // Save deployment info
  const deploymentFile = path.join(abiDir, "deployment.json");
  fs.writeFileSync(deploymentFile, JSON.stringify(deploymentInfo, null, 2));

  console.log("\n✅ Deployment complete!\n");
  console.log("📋 Deployment Info:");
  console.log(JSON.stringify(deploymentInfo, null, 2));
  console.log(`\n💾 ABIs saved to: ${abiDir}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
