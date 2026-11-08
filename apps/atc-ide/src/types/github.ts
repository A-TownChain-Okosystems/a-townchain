export interface GitHubUser {
  login: string;
  id: number;
  avatar_url: string;
  name: string;
  html_url: string;
  public_repos: number;
}

export interface GitHubRepo {
  id: number;
  name: string;
  full_name: string;
  private: boolean;
  html_url: string;
  description: string | null;
  default_branch: string;
  updated_at: string;
  pushed_at?: string;
}

export interface GitHubFilePayload {
  path: string;
  content: string; // utf-8 content or base64
}

export interface GitHubPushRequest {
  owner: string;
  repo: string;
  branch?: string;
  message: string;
  files: { path: string; content: string }[];
}

export interface GitHubCommitItem {
  sha: string;
  commit: {
    message: string;
    author: {
      name: string;
      date: string;
    };
  };
  html_url: string;
}
