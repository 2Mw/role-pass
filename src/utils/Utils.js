import sha256 from 'crypto-js/sha256'

export default {
    hash: function(s) {
        let salt_l = 'kdasjnd93!@$%$YTHFGB!@QWADSCZDasd3][./';
        let salt_r = `sAW'=2O:PzCjHQ/Jy$bYdI>T{K3E1N%hopFgl<.S`;
        return sha256(`${salt_l}${s}${salt_r}`).toString();
    }
}