// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface RequirementCategory {
  id: string;
  title: string;
  packages: string[];
}

export const ATC_OS_REQUIREMENTS: RequirementCategory[] = [
  {
    id: "core",
    title: "Gateway & RPC Core",
    packages: [
      "fastapi>=0.100.0",
      "uvicorn[standard]>=0.23.0",
      "pydantic>=2.0",
      "redis>=4.6.0",
      "python-dotenv>=1.0.0"
    ]
  },
  {
    id: "ai-ml",
    title: "Aurora AI Model Runtime",
    packages: [
      "llama-cpp-python>=0.2.0",
      "onnxruntime>=1.16.0",
      "transformers>=4.30.0",
      "torch>=2.0.0"
    ]
  },
  {
    id: "blockchain",
    title: "A-TownChain Node SDK",
    packages: [
      "web3>=6.0.0",
      "eth-account>=0.9.0",
      "pycryptodome>=3.18.0",
      "ipfshttpclient>=0.8.0"
    ]
  },
  {
    id: "metrics",
    title: "Telemetry & SRE",
    packages: [
      "prometheus-client>=0.17.0",
      "psutil>=5.9.0",
      "sentry-sdk>=1.30.0"
    ]
  },
  {
    id: "testing",
    title: "Smart Contracts & Verification",
    packages: [
      "pytest>=7.0",
      "pytest-asyncio>=0.21.0",
      "brownie>=1.19.0"
    ]
  }
];
