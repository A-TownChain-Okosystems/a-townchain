import { GitHubUser, GitHubRepo, GitHubCommitItem, GitHubPushRequest } from "../types/github";

export interface GitHubAuthState {
  token: string | null;
  user: GitHubUser | null;
  authMethod: "token" | "oauth" | null;
}

const TOKEN_KEY = "atos-github-token";
const USER_KEY = "atos-github-user";
const AUTH_METHOD_KEY = "atos-github-auth-method";

export const getStoredGitHubAuth = (): GitHubAuthState => {
  try {
    const token = localStorage.getItem(TOKEN_KEY);
    const userStr = localStorage.getItem(USER_KEY);
    const authMethod = localStorage.getItem(AUTH_METHOD_KEY) as "token" | "oauth" | null;
    const user = userStr ? JSON.parse(userStr) : null;
    return { token, user, authMethod };
  } catch {
    return { token: null, user: null, authMethod: null };
  }
};

export const saveGitHubAuth = (token: string, user: GitHubUser, method: "token" | "oauth") => {
  localStorage.setItem(TOKEN_KEY, token);
  localStorage.setItem(USER_KEY, JSON.stringify(user));
  localStorage.setItem(AUTH_METHOD_KEY, method);
};

export const clearGitHubAuth = () => {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
  localStorage.removeItem(AUTH_METHOD_KEY);
};

export const fetchGitHubUser = async (token: string): Promise<GitHubUser> => {
  const res = await fetch("/api/github/user", {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err.message || err.error || `HTTP ${res.status}: Failed to fetch user`);
  }

  return await res.json();
};

export const fetchUserRepos = async (token: string): Promise<GitHubRepo[]> => {
  const res = await fetch("/api/github/repos", {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err.message || err.error || `HTTP ${res.status}: Failed to fetch repos`);
  }

  return await res.json();
};

export const createGitHubRepo = async (
  token: string,
  data: { name: string; description?: string; isPrivate?: boolean; autoInit?: boolean }
): Promise<GitHubRepo> => {
  const res = await fetch("/api/github/repos", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(data),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err.message || err.error || `HTTP ${res.status}: Failed to create repository`);
  }

  return await res.json();
};

export const fetchRepoCommits = async (
  token: string,
  owner: string,
  repo: string
): Promise<GitHubCommitItem[]> => {
  const res = await fetch(`/api/github/repos/${owner}/${repo}/commits`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err.message || err.error || `HTTP ${res.status}: Failed to fetch commits`);
  }

  return await res.json();
};

export const pushToGitHubRepo = async (
  token: string,
  params: GitHubPushRequest
): Promise<{ success: boolean; commitSha: string; branch: string; url: string; message: string }> => {
  const res = await fetch(`/api/github/repos/${params.owner}/${params.repo}/push`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify({
      branch: params.branch || "main",
      message: params.message,
      files: params.files,
    }),
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err.message || err.error || `HTTP ${res.status}: Failed to push changes`);
  }

  return await res.json();
};

export const pullFromGitHubRepo = async (
  token: string,
  owner: string,
  repo: string,
  branch: string = "main"
): Promise<{ files: { name: string; path: string; content: string }[]; branch: string; count: number }> => {
  const res = await fetch(`/api/github/repos/${owner}/${repo}/contents?ref=${encodeURIComponent(branch)}`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (!res.ok) {
    const err = await res.json().catch(() => ({}));
    throw new Error(err.message || err.error || `HTTP ${res.status}: Failed to pull files`);
  }

  return await res.json();
};
