import {
    KeysClient,
    ConfusClient,
} from "../grpc/VpnaasServiceClientPb";
import {
    User,
    UserConfig,
    UserPubkey,
    Pubkey,
} from "../grpc/vpnaas_pb";

export async function getConfig(username: string): Promise<UserConfig> {
    return new Promise<UserConfig>((resolve, reject) => {
        new ConfusClient("https://vpnaas.mekstack.ru").get_config(
            new User().setUsername(username),
            {},
            (err, config) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(config);
                }
            }
        );
    });
}

export async function setPubkey(
    username: string,
    accessToken: string,
    pubkey: Pubkey
): Promise<void> {
    return new Promise<void>((resolve, reject) => {
        new KeysClient("https://vpnaas.mekstack.ru").set_pubkey(
            new UserPubkey()
                .setUser(new User().setUsername(username))
                .setPubkey(pubkey),
            { Authorization: `Bearer ${accessToken}` },
            (err, _) => {
                if (err) {
                    reject(err);
                } else {
                    resolve();
                }
            }
        );
    });
}
