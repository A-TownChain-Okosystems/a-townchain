// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { Router, Request, Response } from 'express';
import { requireAuth, AuthRequest } from '../middleware/auth.ts';
import { db } from '../db/index.ts';
import { users } from '../db/schema.ts';
import { eq } from 'drizzle-orm';
import fetch from 'node-fetch';

const router = Router();

router.get('/auth/url', requireAuth, async (req: AuthRequest, res: Response) => {
  const redirectUri = `${process.env.APP_URL}/api/notion/auth/callback`;
  const clientId = process.env.NOTION_CLIENT_ID;

  if (!clientId || !process.env.NOTION_CLIENT_SECRET) {
    return res.status(500).json({ error: 'Notion configuration error' });
  }

  // Use uid as state to associate token later
  const state = req.user?.uid;

  const authUrl = `https://api.notion.com/v1/oauth/authorize?owner=user&client_id=${clientId}&redirect_uri=${encodeURIComponent(
    redirectUri
  )}&response_type=code${state ? `&state=${encodeURIComponent(state)}` : ''}`;

  res.json({ url: authUrl });
});

router.get('/auth/callback', async (req: Request, res: Response) => {
  const { code, state, error } = req.query;

  if (error) {
    return res.status(400).send(`Authentication failed: ${error}`);
  }

  const clientId = process.env.NOTION_CLIENT_ID;
  const clientSecret = process.env.NOTION_CLIENT_SECRET;
  const redirectUri = `${process.env.APP_URL}/api/notion/auth/callback`;

  try {
    const encoded = Buffer.from(`${clientId}:${clientSecret}`).toString('base64');
    
    // Using native fetch since it's available in Node 18+ (but wait, we have node-fetch? No, Node 18+ fetch is available globally. Let's use global fetch).
    const response = await global.fetch('https://api.notion.com/v1/oauth/token', {
      method: 'POST',
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        Authorization: `Basic ${encoded}`,
      },
      body: JSON.stringify({
        grant_type: 'authorization_code',
        code: code,
        redirect_uri: redirectUri,
      }),
    });

    const data = await response.json();

    if (!response.ok) {
      console.error('Notion token error', data);
      throw new Error('Failed to exchange token');
    }

    if (state && typeof state === 'string') {
      const uid = state as string;

      // Update user setting
      const existingUser = await db.select().from(users).where(eq(users.uid, uid)).limit(1);
      
      if (existingUser.length === 0) {
        // If user doesn't exist yet, we could insert, but typically they should exist.
        // Or we just insert here.
        await db.insert(users).values({ uid, email: '', notionAccessToken: data.access_token });
      } else {
        await db.update(users).set({ notionAccessToken: data.access_token }).where(eq(users.uid, uid));
      }
    }

    // Return HTML to postMessage
    res.send(`
      <html>
        <body>
          <script>
            if (window.opener) {
              window.opener.postMessage({ type: 'OAUTH_AUTH_SUCCESS' }, '*');
              window.close();
            } else {
              window.location.href = '/';
            }
          </script>
          <p>Authentication successful. This window should close automatically.</p>
        </body>
      </html>
    `);

  } catch (err) {
    console.error('Notion auth error', err);
    res.status(500).send('Authentication failed');
  }
});

// A route to get user's notion pages
router.get('/pages', requireAuth, async (req: AuthRequest, res: Response) => {
  const uid = req.user?.uid;
  if (!uid) return res.status(401).json({ error: 'Unauthorized' });

  const existingUser = await db.select().from(users).where(eq(users.uid, uid)).limit(1);
  const token = existingUser[0]?.notionAccessToken;

  if (!token) {
    return res.status(400).json({ error: 'Notion not connected' });
  }

  try {
    const response = await global.fetch('https://api.notion.com/v1/search', {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Notion-Version': '2022-06-28',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        filter: {
          property: 'object',
          value: 'page'
        }
      })
    });

    const data = await response.json();
    if (!response.ok) {
       // if token invalid, might clear it in db
       if (response.status === 401) {
           await db.update(users).set({ notionAccessToken: null }).where(eq(users.uid, uid));
       }
       throw new Error(data.message || 'Notion API Error');
    }

    res.json(data);
  } catch(err: any) {
    res.status(500).json({ error: err.message });
  }
});

export default router;
