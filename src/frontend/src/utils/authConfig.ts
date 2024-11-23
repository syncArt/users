import {
  AuthClientCreateOptions,
  AuthClientLoginOptions
} from "@dfinity/auth-client";

export const getIdentityProvider = (): string | undefined => {
  let idpProvider;
  if (typeof window !== "undefined") {
    const isLocal = process.env.DFX_NETWORK !== "ic";
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
    if (isLocal && isSafari) {
      idpProvider = `http://localhost:4943/?canisterId=bkyz2-fmaaa-aaaaa-qaaaq-cai`; //Internet identity z User Identity
    } else if (isLocal) {
      idpProvider = `http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943`; //Internet identity z User Identity
    }
  }
  return idpProvider;
};

export const getDerivationOrigin = () => {
  let derivationOrigin;
  if (typeof window !== "undefined") {
    const isLocal = process.env.DFX_NETWORK !== "ic";
    if (isLocal) {
      derivationOrigin = `http://asrmz-lmaaa-aaaaa-qaaeq-cai.localhost:4943`;
    } else {
      derivationOrigin = `https://xufwy-sqaaa-aaaan-qmwcq-cai.icp0.io/`;
    }
  }
  return derivationOrigin;
};

export type AuthConfig = {
  createOptions: AuthClientCreateOptions;
  loginOptions: AuthClientLoginOptions;
};

export const defaultOptions: AuthConfig = {
  /**
   *  @type {AuthClientCreateOptions}
   */
  createOptions: {
    idleOptions: {
      disableIdle: true
    }
  },
  /**
   * @type {AuthClientLoginOptions}
   */
  loginOptions: {
    identityProvider: getIdentityProvider(),
    windowOpenerFeatures:
      "toolbar=0,location=0,menubar=0,width=500,height=500,left=100,top=100",
    derivationOrigin: getDerivationOrigin()
  }
};
