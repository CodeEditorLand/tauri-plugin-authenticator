// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
class Authenticator {
    async init() {
        return await window.__TAURI_INVOKE__("plugin:authenticator|init_auth");
    }
    async register(challenge, application) {
        return await window.__TAURI_INVOKE__("plugin:authenticator|register", {
            timeout: 10000,
            challenge,
            application,
        });
    }
    async verifyRegistration(challenge, application, registerData, clientData) {
        return await window.__TAURI_INVOKE__("plugin:authenticator|verify_registration", {
            challenge,
            application,
            registerData,
            clientData,
        });
    }
    async sign(challenge, application, keyHandle) {
        return await window.__TAURI_INVOKE__("plugin:authenticator|sign", {
            timeout: 10000,
            challenge,
            application,
            keyHandle,
        });
    }
    async verifySignature(challenge, application, signData, clientData, keyHandle, pubkey) {
        return await window.__TAURI_INVOKE__("plugin:authenticator|verify_signature", {
            challenge,
            application,
            signData,
            clientData,
            keyHandle,
            pubkey,
        });
    }
}

export { Authenticator };
//# sourceMappingURL=index.mjs.map
