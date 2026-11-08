import React, { createContext, useContext, useState, useEffect } from 'react';
import { User } from 'firebase/auth';
import { googleSignIn, logout, initAuth, googleAuthProvider } from '../lib/firebase';

// Request standard Google Workspace scopes
googleAuthProvider.addScope('https://www.googleapis.com/auth/drive');
googleAuthProvider.addScope('https://www.googleapis.com/auth/drive.file');
googleAuthProvider.addScope('https://www.googleapis.com/auth/drive.readonly');
googleAuthProvider.addScope('https://www.googleapis.com/auth/calendar.events');
googleAuthProvider.addScope('https://www.googleapis.com/auth/spreadsheets');
googleAuthProvider.addScope('https://www.googleapis.com/auth/spreadsheets.readonly');
googleAuthProvider.addScope('https://www.googleapis.com/auth/gmail.readonly');
googleAuthProvider.addScope('https://www.googleapis.com/auth/documents.readonly');
googleAuthProvider.addScope('https://www.googleapis.com/auth/documents');

interface GoogleWorkspaceContextType {
  user: User | null;
  accessToken: string | null;
  signIn: () => Promise<void>;
  signOut: () => Promise<void>;
  isAuthenticated: boolean;
}

const GoogleWorkspaceContext = createContext<GoogleWorkspaceContextType | null>(null);

export const GoogleWorkspaceProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [accessToken, setAccessToken] = useState<string | null>(null);
  
  useEffect(() => {
    const unsubscribe = initAuth(
      (authUser, token) => {
        setUser(authUser);
        setAccessToken(token);
      },
      () => {
        setUser(null);
        setAccessToken(null);
      }
    );
    return () => unsubscribe();
  }, []);

  const signIn = async () => {
    try {
      const result = await googleSignIn();
      if (result) {
        setUser(result.user);
        setAccessToken(result.accessToken);
      }
    } catch (e) {
      console.error("Sign in failed", e);
    }
  };

  const signOut = async () => {
    await logout();
    setUser(null);
    setAccessToken(null);
  };

  return (
    <GoogleWorkspaceContext.Provider
      value={{
        user,
        accessToken,
        signIn,
        signOut,
        isAuthenticated: !!user && !!accessToken,
      }}
    >
      {children}
    </GoogleWorkspaceContext.Provider>
  );
};

export const useGoogleWorkspace = () => {
  const context = useContext(GoogleWorkspaceContext);
  if (!context) {
    throw new Error('useGoogleWorkspace must be used within a GoogleWorkspaceProvider');
  }
  return context;
};
