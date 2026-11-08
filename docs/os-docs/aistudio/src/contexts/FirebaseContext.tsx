import React, { createContext, useContext, useEffect, useState } from 'react';
import { User } from 'firebase/auth';
import { auth, initAuth, googleSignIn, logout, getAccessToken } from '../lib/firebase';

interface FirebaseContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  signIn: () => Promise<void>;
  signOut: () => Promise<void>;
  accessToken: string | null;
}

const FirebaseContext = createContext<FirebaseContextType>({
  user: null,
  isAuthenticated: false,
  isLoading: true,
  signIn: async () => {},
  signOut: async () => {},
  accessToken: null,
});

export const useFirebase = () => useContext(FirebaseContext);

export const FirebaseProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [accessToken, setAccessToken] = useState<string | null>(null);

  useEffect(() => {
    const unsubscribe = initAuth(
      (authUser, token) => {
        setUser(authUser);
        setIsAuthenticated(true);
        setAccessToken(token);
        setIsLoading(false);
      },
      () => {
        setUser(null);
        setIsAuthenticated(false);
        setAccessToken(null);
        setIsLoading(false);
      }
    );

    return () => unsubscribe();
  }, []);

  const signIn = async () => {
    try {
      setIsLoading(true);
      const result = await googleSignIn();
      if (result) {
        setUser(result.user);
        setAccessToken(result.accessToken);
        setIsAuthenticated(true);
      }
    } catch (error) {
      console.error('Failed to sign in:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const signOut = async () => {
    try {
      setIsLoading(true);
      await logout();
      setUser(null);
      setAccessToken(null);
      setIsAuthenticated(false);
    } catch (error) {
      console.error('Failed to sign out:', error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <FirebaseContext.Provider
      value={{
        user,
        isAuthenticated,
        isLoading,
        signIn,
        signOut,
        accessToken,
      }}
    >
      {children}
    </FirebaseContext.Provider>
  );
};
