// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import { initializeApp, getApps } from 'firebase-admin/app';
import { getAuth } from 'firebase-admin/auth';
import type { ServiceAccount } from 'firebase-admin/app';

// MUST be static import
import firebaseConfig from '../../firebase-applet-config.json' with { type: "json" };

if (!getApps().length) {
  initializeApp({
    projectId: firebaseConfig.projectId || process.env.GOOGLE_CLOUD_PROJECT,
  });
}

export const adminAuth = getAuth();
