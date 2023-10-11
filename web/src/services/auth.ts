import jwtDecode from "jwt-decode";
import { UserManager } from 'oidc-client-ts';

const userManager = new UserManager({
    authority: 'https://vault.mekstack.ru/v1/identity/oidc/provider/hse',
    metadata: {
        issuer: 'https://vault.mekstack.ru/v1/identity/oidc/provider/hse',
        jwks_uri: 'https://vault.mekstack.ru/v1/identity/oidc/provider/hse/.well-known/keys',
        token_endpoint: 'https://vault.mekstack.ru/v1/identity/oidc/provider/hse/token',
        authorization_endpoint: 'https://vault.mekstack.ru/ui/vault/identity/oidc/provider/hse/authorize'
    },
    client_id: 'OH6r6tUSHseRZecMVxHLcjrlBh2IyNqa',
    redirect_uri: window.location.origin,
    response_type: 'code',
    scope: 'openid user',
});

interface VPNaaSToken {
    username: string;
    exp: number;
}

export async function getAccessTokenAfterSignin(): Promise<string | undefined> {
    const urlParams = new URLSearchParams(window.location.search);

    if (urlParams.has('state')) {
        try {
            const user = await userManager.signinRedirectCallback();
            const accessToken = user.id_token;
            if (accessToken) {
                localStorage.setItem('accessToken', accessToken);
            }
            return accessToken;
        } catch (err) {
            console.error('Error handling redirect callback:', err);
            throw err;
        } finally {
            window.location.href = window.location.origin;
        }
    }

    return undefined;
}

export function getAccessTokenFromLocalStorage(): string | null {
    return localStorage.getItem("accessToken");
}

export function getUsernameFromAccessToken(accessToken: string): string {
    return jwtDecode<VPNaaSToken>(accessToken).username;
}

export function isTokenExpired(accessToken: string): boolean {
    const decodedToken = jwtDecode<VPNaaSToken>(accessToken);
    const currentTimestamp = Math.floor(Date.now() / 1000);
    return decodedToken.exp <= currentTimestamp;
}

export function checkTokenExpiry(): void {
    const accessToken = localStorage.getItem('accessToken');
    if (accessToken && isTokenExpired(accessToken)) {
        localStorage.removeItem('accessToken');
    }
}

export function login(): void {
    userManager.signinRedirect();
}
