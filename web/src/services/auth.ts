import jwtDecode from "jwt-decode";

interface VPNaaSToken {
    username: string;
    exp: number;
}

export function checkForAccessTokenInHash(): string | null {
    const hashParams = new URLSearchParams(window.location.hash.substring(1));

    if (hashParams.has("access_token")) {
        const accessToken = hashParams.get("access_token");
        localStorage.setItem("accessToken", accessToken);

        // Remove access_token from the hash fragment
        const newHashParams = new URLSearchParams(hashParams);
        newHashParams.delete("access_token");
        window.location.hash = newHashParams.toString();

        return accessToken;
    }
    return null;
}

export function getAccessTokenFromLocalStorage(): string | null {
    return localStorage.getItem("accessToken");
}

export function getUsernameFromAccessToken(accessToken: string): string {
    return jwtDecode<VPNaaSToken>(accessToken).username;
}

export function login(): void {
    const redirectBackUrl = encodeURIComponent(window.location.href);
    const authUrl = `https://auth.mekstack.ru/login?redirect_back_url=${redirectBackUrl}`;
    window.location.href = authUrl;
}
