// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
export interface ArchitectureLayer {
  id: number;
  name: string;
  purpose: string;
  components: string[];
  iconName: string;
}

export type IconName = "Scale" | "Brain" | "FlaskConical" | "TrendingUp" | "Gavel" | "Users" | "Server" | "Shield" | "Globe" | "CheckCircle";
