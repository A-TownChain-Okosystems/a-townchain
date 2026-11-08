// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
fetch('http://localhost:3000/api/knowledge').then(r=>r.json()).then(d=>console.log(d.code?.length, "files loaded")).catch(e=>console.error(e));
