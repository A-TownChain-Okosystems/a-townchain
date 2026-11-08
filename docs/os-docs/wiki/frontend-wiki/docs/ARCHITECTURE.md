# 🏗️ Architektur — atc-frontend

Das Frontend-Repository `atc-frontend` bildet die Layer-10 Präsentationsschicht im KAI-OS Ökosystem.

## Schichtenmodell

1. **User Interface Layer**: Desktop Window Manager, Widget Grid, Neon Dashboard.
2. **Application State Layer**: Event-Bus, Session Vault, Active App Manager.
3. **Network & Transport Layer**: REST & WebSocket Protocol Client (`assets/js/api.js`).

## Kommunikation

```
+-------------------+        WebSocket / REST       +-------------------+
|   atc-frontend    | <--------------------------> |   atc-gateway     |
|   (Browser / L10) |                              |   (Port 4000)     |
+-------------------+                              +-------------------+
```
