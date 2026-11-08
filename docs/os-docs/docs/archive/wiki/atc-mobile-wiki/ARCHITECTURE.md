# ARCHITECTURE.md — atc-mobile
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── __init__.py
├── ecdsa.atc
├── keygen.atc
├── requirements.txt
├── src/
├── wallet/
│   ├── __init__.py
│   ├── biometric_auth.atc
│   ├── keygen.py
│   └── wallet.atc
└── wallet_api.atc
```

## Module Descriptions
- **src/**: Core mobile framework, application lifecycle management, and event handler loops.
- **wallet/**: Cryptographic mobile wallet subsystem including keypair generation (`keygen.py`), address management, transaction signing, and biometric authentication (`biometric_auth.atc`).
- **requirements.txt**: Python package manifest specifying runtime and cryptographic dependencies.

## Build System
Python 3.10+ packaging toolchain (`setuptools`). Mobile compilation and bundling supported via BeeWare / Kivy / Briefcase for iOS and Android.

## Dependencies
Python 3.10+, `cryptography`, `ecdsa`, `requests`, `pydantic`, OS biometric authentication native bindings.
