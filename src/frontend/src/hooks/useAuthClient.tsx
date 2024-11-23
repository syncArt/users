// @ts-nocheck
"use client";

import { AuthClient } from "@dfinity/auth-client";
import React, { createContext, useContext, useEffect, useState } from "react";
import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
import {
  canisterId as usersInterRsCanisterId,
  idlFactory as usersInterRsIdl
} from "declarations/users-inter-rs";
import { _SERVICE as UsersInterRsService } from "declarations/users-inter-rs/users-inter-rs.did";
import { Principal } from "@dfinity/principal";

type AuthContextType = {
  isAuthenticated: boolean;
  login: (func?: () => void) => void;
  logout: () => void;
  authClient: AuthClient | null;
  identity: Identity | null;
  principal: Principal | null;
  usersInterRsActor: ActorSubclass<UsersInterRsService>;
  loading: boolean;
};

const AuthContext = createContext<AuthContextType | null>(null);

export const getIdentityProvider = () => {
  let idpProvider;
  if (typeof window !== "undefined") {
    const isLocal = process.env.DFX_NETWORK !== "ic";
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
    if (isLocal && isSafari) {
      idpProvider = `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
    } else if (isLocal) {
      idpProvider = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
    }
  }
  return idpProvider;
};

export const getDerivationOrigin = () => {
  let derivationOrigin;
  if (typeof window !== "undefined") {
    const isLocal = process.env.DFX_NETWORK !== "ic";
    if (isLocal) {
      derivationOrigin = `http://${process.env.CANISTER_ID_FRONTEND}.localhost:4943`;
    } else {
      derivationOrigin = "https://qmg3k-zyaaa-aaaan-qm24a-cai.icp0.io/";
    }
  }
  return derivationOrigin;
};

export const defaultOptions = {
  createOptions: {
    idleOptions: {
      disableIdle: true
    }
  },
  loginOptions: {
    identityProvider: getIdentityProvider(),
    windowOpenerFeatures:
      "toolbar=0,location=0,menubar=0,width=500,height=500,left=100,top=100",
    derivationOrigin: getDerivationOrigin()
  }
};

export const useAuthClient = (options = defaultOptions) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [authClient, setAuthClient] = useState<AuthClient | null>(null);
  const [identity, setIdentity] = useState(null);
  const [principal, setPrincipal] = useState(null);
  const [usersInterRsActor, setUsersInterRsActor] =
    useState<UsersInterRsService | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    AuthClient.create(options.createOptions).then(async (client) => {
      await updateClient(client);
      setLoading(false);
    });
  }, []);

  const updateClient = async (client: AuthClient) => {
    const isAuthenticated = await client.isAuthenticated();
    setIsAuthenticated(isAuthenticated);

    const identity = client.getIdentity();
    setIdentity(identity);

    const principal = identity.getPrincipal();
    setPrincipal(principal);

    setAuthClient(client);

    const agent = new HttpAgent({ identity });
    if (process.env.DFX_NETWORK !== "ic") {
      agent.fetchRootKey();
    }

    const actor = Actor.createActor<UsersInterRsService>(usersInterRsIdl, {
      agent,
      canisterId: usersInterRsCanisterId
    });
    setUsersInterRsActor(actor);
  };

  const logout = async () => {
    if (authClient) {
      await authClient.logout();
      setIsAuthenticated(false);
      setIdentity(null);
      setPrincipal(null);
    }
  };

  return {
    isAuthenticated,
    login: async (callback?: () => void) => {
      await authClient?.login({
        ...options.loginOptions,
        onSuccess: () => {
          updateClient(authClient!);
          if (callback) callback();
        },
        onError: () => console.error("Couldn't authorize user.")
      });
    },
    logout,
    authClient,
    loading,
    identity,
    principal,
    usersInterRsActor
  };
};

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const auth = useAuthClient();

  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
};

export const useApi = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
};
