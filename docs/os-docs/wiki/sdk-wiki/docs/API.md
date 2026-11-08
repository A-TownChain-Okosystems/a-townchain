# 🔌 API Reference — atc-sdk

---

## 1. Common Methods

### `getBalance(address: string): Promise<BigInt>`
Ermittelt den Kontostand für die angegebene Adresse.

### `deployContract(bytecode: Bytes, params: Array): Promise<TxHash>`
Signiert und übermittelt eine Smart Contract Deployment-Transaktion.
