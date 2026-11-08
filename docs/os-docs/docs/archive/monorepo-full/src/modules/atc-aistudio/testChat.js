// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
async function test() {
  const res = await fetch('https://app.base44.com/api/agents/6a1d37209d7f46ac065d39c5/conversations?api_key=ddc7d8f665684878afcc73c2b8b62b7a', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ message: "Hello" })
  });
  console.log(res.status, await res.text());
}
test();
