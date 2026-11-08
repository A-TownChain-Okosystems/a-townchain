// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
async function run() {
  const urls = [
    `https://app.base44.com/api/agents/6a1d37209d7f46ac065d39c5?api_key=ddc7d8f665684878afcc73c2b8b62b7a`,
    `https://app.base44.com/api/agents/6a1d37209d7f46ac065d39c5?token=ddc7d8f665684878afcc73c2b8b62b7a`
  ];
  const headersList = [
    { 'x-api-key': 'ddc7d8f665684878afcc73c2b8b62b7a' },
    { 'Authorization': 'token ddc7d8f665684878afcc73c2b8b62b7a' }
  ];

  for(let url of urls) {
     console.log('trying', url);
     const res = await fetch(url);
     const text = await res.text();
     if(res.status === 200) {
       console.log(text);
       return;
     } else {
       console.log(res.status, text);
     }
  }

  for(let headers of headersList) {
     console.log('trying headers', headers);
     const res = await fetch('https://app.base44.com/api/agents/6a1d37209d7f46ac065d39c5', { headers });
     const text = await res.text();
     if(res.status === 200) {
       console.log(text);
       return;
     } else {
       console.log(res.status, text);
     }
  }
}
run();
