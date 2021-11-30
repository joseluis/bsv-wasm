#[cfg(test)]
mod tests {
    extern crate wasm_bindgen_test;
    use bsv_wasm::Script;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    // #[test]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    // fn to_hex_string() {
    //   let script_bytes = hex::decode("006a2231394878696756345179427633744870515663554551797131707a5a56646f4175744c8d2453485541207768616c65206170706561726564efbc81efbc810a4d61796265205348554120736861726befbc81f09f98aff09f98af0a0a68747470733a2f2f7477657463682e6170702f742f643435653233303233383762616235643138663534666566643736633461353462356466636338316461396436623133643832303335333838313064633565630a746578742f706c61696e04746578741f7477657463685f7477746578745f313631393336303532303332352e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c046e756c6c07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652438613262346330622d636531362d346166342d393932312d386638393334343436383938017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f45434453412231513533564e7853316e647a56444431623834316545795a4458574c6a42735167694c5848384f4c685141527132365371784d56494852554449624b4e45686d49424e6e6e362f376a70786466685731434b61387a63356c3043497471425177557a47734b4b4f2b6e4e4c33444c45424c2f7467433657666f46413d").unwrap();
    //   let script = Script::from_bytes(script_bytes);

    //   assert_eq!(script.to, "006a2231394878696756345179427633744870515663554551797131707a5a56646f4175744c8d2453485541207768616c65206170706561726564efbc81efbc810a4d61796265205348554120736861726befbc81f09f98aff09f98af0a0a68747470733a2f2f7477657463682e6170702f742f643435653233303233383762616235643138663534666566643736633461353462356466636338316461396436623133643832303335333838313064633565630a746578742f706c61696e04746578741f7477657463685f7477746578745f313631393336303532303332352e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c046e756c6c07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652438613262346330622d636531362d346166342d393932312d386638393334343436383938017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f45434453412231513533564e7853316e647a56444431623834316545795a4458574c6a42735167694c5848384f4c685141527132365371784d56494852554449624b4e45686d49424e6e6e362f376a70786466685731434b61387a63356c3043497471425177557a47734b4b4f2b6e4e4c33444c45424c2f7467433657666f46413d");
    // }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_21e8_script_hex_debug() {
        let script = Script::from_hex("20d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a20221e8825479a87c7f758875ac").unwrap();

        assert_eq!(
            script.to_extended_asm_string().unwrap(),
            "OP_PUSH 32 d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 OP_PUSH 2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_21e8_script_hex() {
        let script = Script::from_hex("20d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a20221e8825479a87c7f758875ac").unwrap();

        assert_eq!(
            script.to_asm_string().unwrap(),
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_asm_op_return_script() {
        let script = Script::from_hex("006a4cb47b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d").unwrap();

        assert_eq!(script.to_asm_string().unwrap(), "0 OP_RETURN 7b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn from_21e8_asm_string() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        assert_eq!(
            script.to_asm_string().unwrap(),
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
        assert_eq!(
            script.to_extended_asm_string().unwrap(),
            "OP_PUSH 32 d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 OP_PUSH 2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn from_op_return_asm_string() {
        let script = Script::from_asm_string("0 OP_RETURN 7b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d").unwrap();

        assert_eq!(script.to_asm_string().unwrap(), "0 OP_RETURN 7b227573657248616e646c65223a226c75636b787878222c226368616e6e656c223a226d61746368222c226368616e6e656c4964223a2264757374222c2277696e6e65724964223a2239383333323836362d636435372d343166332d393537632d636433376231666237643738222c2275736572496d616765223a2268747470733a2f2f636c6f75642e68616e64636173682e696f2f75736572732f70726f66696c65506963747572652f6c75636b787878227d");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn from_custom_asm_string() {
        let script = Script::from_asm_string("OP_NOP OP_0 ff OP_0 OP_PICK OP_2 OP_ROLL OP_DROP OP_1 OP_ROLL OP_DROP OP_NOP OP_1 OP_PICK OP_0 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_NUMEQUAL OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_1 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_LESSTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_2 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_GREATERTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_0 OP_ENDIF OP_ENDIF OP_ENDIF").unwrap();

        assert_eq!(script.to_extended_asm_string().unwrap(), "OP_NOP OP_0 OP_PUSH 1 ff OP_0 OP_PICK OP_2 OP_ROLL OP_DROP OP_1 OP_ROLL OP_DROP OP_NOP OP_1 OP_PICK OP_0 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_NUMEQUAL OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_1 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_LESSTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_2 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_GREATERTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_0 OP_ENDIF OP_ENDIF OP_ENDIF");
        assert_eq!(script.to_asm_string().unwrap(), "OP_NOP 0 ff 0 OP_PICK OP_2 OP_ROLL OP_DROP OP_1 OP_ROLL OP_DROP OP_NOP OP_1 OP_PICK 0 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_NUMEQUAL OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_1 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_LESSTHAN OP_NIP OP_NIP OP_NIP OP_ELSE OP_1 OP_PICK OP_2 OP_EQUAL OP_IF OP_2 OP_PICK OP_1 OP_PICK OP_GREATERTHAN OP_NIP OP_NIP OP_NIP OP_ELSE 0 OP_ENDIF OP_ENDIF OP_ENDIF");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn from_p2pkh_asm_string() {
        let script = Script::from_asm_string("OP_DUP OP_HASH160 6fa5502ea094d59576898b490d866b32a61b89f6 OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert_eq!(script.to_asm_string().unwrap(), "OP_DUP OP_HASH160 6fa5502ea094d59576898b490d866b32a61b89f6 OP_EQUALVERIFY OP_CHECKSIG");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn from_problematic_asm_string() {
        let script = Script::from_asm_string("OP_RETURN 026d02 0568656c6c6f").unwrap();

        assert_eq!(script.to_asm_string().unwrap(), "OP_RETURN 026d02 0568656c6c6f");
    }

    #[test]
    fn throw_error_with_invalid_hex() {
        let script = Script::from_asm_string("OP_RETURN 026d02 0568656c6c6fzz");

        assert!(script.is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn from_standard_21e8_asm_format() {
        let script =
            Script::from_asm_string("0a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e75846144 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        assert_eq!(
            script.to_asm_string().unwrap(),
            "0a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e75846144 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn another_hex_21e8_script() {
        let script = Script::from_hex("200a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e758461440221e8825479a87c7f758875ac").unwrap();

        assert_eq!(
            script.to_asm_string().unwrap(),
            "0a40eda5ff94de646c3928e4a8eff097feeb283d124b0e871b24962e75846144 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG"
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn encode_pushdata_test() {
        let test_data: Vec<u8> = vec![0; 11];

        let pushdata = Script::encode_pushdata(&test_data).unwrap();
        assert_eq!(pushdata.len(), 12);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn encode_pushdata_2_test() {
        let test_data: Vec<u8> = vec![0; 1024];

        let pushdata = Script::encode_pushdata(&test_data).unwrap();
        assert_eq!(pushdata.len(), 1027);
    }

    #[test]
    // Cant run in wasm, data size too beeg
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn encode_pushdata_4_test() {
        let test_data: Vec<u8> = vec![0; 3_000_000_000];

        let pushdata = Script::encode_pushdata(&test_data).unwrap();
        assert_eq!(pushdata.len(), 3000000005);
    }
}
