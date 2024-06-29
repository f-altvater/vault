#[cfg(test)]
mod tests {
    use crate::cryptography::{decrypt_text, encrypt_text};

    #[test]
    fn different_encryptions_for_same_text() {
        let text = "test text";
        let master_pw_encryption = encrypt_text(&text, true, true).unwrap();
        let pw_encryption = encrypt_text(&text, false, true).unwrap();
        let text_encryption = encrypt_text(&text, false, false).unwrap();

        assert_ne!(
            master_pw_encryption,
            pw_encryption,
            "\n!!! Encrypted Strings 'master_pw_encryption' and 'pw_encryption' are equal!",
        );
        assert_ne!(
            master_pw_encryption,
            text_encryption,
            "\n!!! Encrypted Strings 'master_pw_encryption' and 'text_encryption' are equal!",
        );
        assert_ne!(
            pw_encryption,
            text_encryption,
            "\n!!! Encrypted Strings 'pw_encryption' and 'text_encryption' are equal!",
        );
    }

    #[test]
    fn different_encryptions_give_same_decryption() {
        let text = "test text";
        let master_pw_encryption = encrypt_text(&text, true, true).unwrap();
        let pw_encryption = encrypt_text(&text, false, true).unwrap();
        let text_encryption = encrypt_text(&text, false, false).unwrap();
        let master_decrypted = decrypt_text(&master_pw_encryption, true, true).unwrap();
        let pw_decrypted = decrypt_text(&pw_encryption, false, true).unwrap();
        let text_decrypted = decrypt_text(&text_encryption, false, false).unwrap();

        assert_eq!(
            text,
            &master_decrypted,
            "\n!!! Text was: {} -- Decrypted Master is: {}",
            text, &master_decrypted
        );
        assert_eq!(
            text,
            &pw_decrypted,
            "\n!!! Text was: {} -- Decrypted PW is: {}",
            text, &pw_decrypted
        );
        assert_eq!(
            text,
            &text_decrypted,
            "\n!!! Text was: {} -- Decrypted Text is: {}",
            text, &text_decrypted
        );
    }

    #[test]
    fn special_chars_work() {
        let text = "-=[];'#,./_+{}:@~<>?\\`¬¦!\"£$%^&*|€";
        let master_pw_encryption = encrypt_text(&text, true, true).unwrap();
        let pw_encryption = encrypt_text(&text, false, true).unwrap();
        let text_encryption = encrypt_text(&text, false, false).unwrap();
        let master_decrypted = decrypt_text(&master_pw_encryption, true, true).unwrap();
        let pw_decrypted = decrypt_text(&pw_encryption, false, true).unwrap();
        let text_decrypted = decrypt_text(&text_encryption, false, false).unwrap();

        assert_eq!(
            text,
            &master_decrypted,
            "\n!!! Text was: {} -- Decrypted Master is: {}",
            text, &master_decrypted
        );
        assert_eq!(
            text,
            &pw_decrypted,
            "\n!!! Text was: {} -- Decrypted PW is: {}",
            text, &pw_decrypted
        );
        assert_eq!(
            text,
            &text_decrypted,
            "\n!!! Text was: {} -- Decrypted Text is: {}",
            text, &text_decrypted
        );
    }

    #[test]
    fn word_of_512_chars() {
        let text = "awednepdfuiyiaqp'kjecmnva;vja#'v;klasd[p]iasd[#pfkadsjd[fouias#[d;kjasdo[dfuiasd#[fdjasdo[dfiuasdl;aso;ufsajfpo nfasddpofuaspfjdsafhasfkadsf;hasdofiasdfhoasd;asdjf;poasf
ahsdoifhasdolfjasldflkasdhjf;bfkj  apsdjfasdhfjasfkdasbfhdasfdnaskfl asfnasdhjfbghdasuikfhjasdlfblaskdfjnasdknfb asdfhiasdhfoisdjfashgfashfhgasdfj dopfhjafasdhfsdahfahgsdfhjasdfhasdlfkaslhfjkasdhfhjkhasdasdkjlfh sdajfhjasdhfhsadfhksadjh lkdjh     ashdfjihasjfhdsafhaskjfh asdipjfhjasdfhjkashfkldjashfahfiuehasfdhfiahy8932743174897347281730201";
        let master_pw_encryption = encrypt_text(&text, true, true).unwrap();
        let pw_encryption = encrypt_text(&text, false, true).unwrap();
        let text_encryption = encrypt_text(&text, false, false).unwrap();
        let master_decrypted = decrypt_text(&master_pw_encryption, true, true).unwrap();
        let pw_decrypted = decrypt_text(&pw_encryption, false, true).unwrap();
        let text_decrypted = decrypt_text(&text_encryption, false, false).unwrap();

        assert_eq!(
            text,
            &master_decrypted,
            "\n!!! Text was: {} -- Decrypted Master is: {}",
            text, &master_decrypted
        );
        assert_eq!(
            text,
            &pw_decrypted,
            "\n!!! Text was: {} -- Decrypted PW is: {}",
            text, &pw_decrypted
        );
        assert_eq!(
            text,
            &text_decrypted,
            "\n!!! Text was: {} -- Decrypted Text is: {}",
            text, &text_decrypted
        );
    }
}