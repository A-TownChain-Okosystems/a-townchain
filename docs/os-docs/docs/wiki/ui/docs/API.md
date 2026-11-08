# Frontend API-Integration

## Basis-URL
```javascript
const API_BASE = 'http://localhost:4000';
```

## Wallet-Balance
```javascript
const res = await fetch(`${API_BASE}/api/wallet/balance/${address}`);
const { balance } = await res.json();
```

## TX senden
```javascript
const res = await fetch(`${API_BASE}/api/wallet/send`, {
  method: 'POST',
  headers: { 'X-API-Key': apiKey, 'X-Signature': sig },
  body: JSON.stringify({ to, amount, nonce })
});
```

## Gemini Chat
```javascript
const res = await fetch(`${API_BASE}/api/ai/chat`, {
  method: 'POST',
  headers: { 'X-API-Key': apiKey },
  body: JSON.stringify({ message: userInput })
});
```
