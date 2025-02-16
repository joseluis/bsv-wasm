#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod tx_criteria_tests {
    use bsv_wasm::*;
    extern crate wasm_bindgen_test;
    use bsv_wasm::TxIn;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn given_transaction_with_no_criteria_match_all_inputs() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_input(&TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::from_asm_string("").unwrap(),
            Some(u32::MAX),
        ));
        tx.add_input(&TxIn::new(
      &hex::decode("ae1bce3baad401f5ce96d6b5f34427a94f4bdd2b4c401298acc54927ac7afdb0").unwrap(),
      2,
      &Script::from_hex("4730440220029fa2e1301bf1073f3dbea9c9ddf797a4a211ef63dc5ab26ce9f21513d12e8d022032af0020d4c07b96969e3e99f228c6cd463ba58e47a9020d3ca8215ac3a5da22412103c134c904118b148d32492cd17d1183088f708a3e4a7429f3260ff51b9e72c6cc").unwrap(),
      Some(u32::MAX),
    ));
        tx.add_input(&TxIn::new(
            &hex::decode("f2e1978486452bd4262f3f51fb54fb50ca55ba3e928c3aabfa27e11a1b230d02").unwrap(),
            0,
            &Script::from_asm_string("").unwrap(),
            Some(u32::MAX),
        ));
        tx.add_input(&TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
            Some(u32::MAX),
        ));

        // Act
        let matches = tx.match_inputs(&MatchCriteria::new());

        assert_eq!(matches.len(), 4)
    }

    #[test]
    #[wasm_bindgen_test]
    fn given_transaction_with_p2pkh_criteria_match_p2pkh_input() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_input(&TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::from_asm_string("").unwrap(),
            Some(u32::MAX),
        ));
        tx.add_input(&TxIn::new(
      &hex::decode("ae1bce3baad401f5ce96d6b5f34427a94f4bdd2b4c401298acc54927ac7afdb0").unwrap(),
      2,
      &Script::from_hex("4730440220029fa2e1301bf1073f3dbea9c9ddf797a4a211ef63dc5ab26ce9f21513d12e8d022032af0020d4c07b96969e3e99f228c6cd463ba58e47a9020d3ca8215ac3a5da22412103c134c904118b148d32492cd17d1183088f708a3e4a7429f3260ff51b9e72c6cc").unwrap(),
      Some(u32::MAX),
    ));
        tx.add_input(&TxIn::new(
            &hex::decode("f2e1978486452bd4262f3f51fb54fb50ca55ba3e928c3aabfa27e11a1b230d02").unwrap(),
            0,
            &Script::from_asm_string("").unwrap(),
            Some(u32::MAX),
        ));
        tx.add_input(&TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
            Some(u32::MAX),
        ));

        // Act
        let criteria = MatchCriteria::new().set_script(&P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap());
        let matches = tx.match_inputs(&criteria);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], 3);
    }

    #[test]
    #[wasm_bindgen_test]
    fn given_transaction_with_p2pkh_and_min_max_sats_criteria_match_single_p2pkh_input() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_input(&TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::from_asm_string("").unwrap(),
            Some(u32::MAX),
        ));
        tx.add_input(&TxIn::new(
      &hex::decode("ae1bce3baad401f5ce96d6b5f34427a94f4bdd2b4c401298acc54927ac7afdb0").unwrap(),
      2,
      &Script::from_hex("4730440220029fa2e1301bf1073f3dbea9c9ddf797a4a211ef63dc5ab26ce9f21513d12e8d022032af0020d4c07b96969e3e99f228c6cd463ba58e47a9020d3ca8215ac3a5da22412103c134c904118b148d32492cd17d1183088f708a3e4a7429f3260ff51b9e72c6cc").unwrap(),
      Some(u32::MAX),
    ));
        tx.add_input(&TxIn::new(
            &hex::decode("f2e1978486452bd4262f3f51fb54fb50ca55ba3e928c3aabfa27e11a1b230d02").unwrap(),
            0,
            &Script::from_asm_string("").unwrap(),
            Some(u32::MAX),
        ));
        tx.add_input(&TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
            Some(u32::MAX),
        ));
        let mut txin = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
            Some(u32::MAX),
        );
        txin.set_satoshis(6000);
        tx.add_input(&txin);

        // Act
        let criteria = MatchCriteria::new()
            .set_min(2000)
            .set_max(7000)
            .set_script(&P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap());
        let matches = tx.match_inputs(&criteria);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], 4);
    }

    #[test]
    #[wasm_bindgen_test]
    fn given_transaction_with_no_criteria_match_all_outputs() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_output(&TxOut::new(
            5000,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(
            400,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(
            9999999,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        // Act
        let matches = tx.match_outputs(&MatchCriteria::new());

        assert_eq!(matches.len(), 4)
    }

    #[test]
    #[wasm_bindgen_test]
    fn given_transaction_with_p2pkh_and_exact_value_return_single() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_output(&TxOut::new(
            5000,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(
            400,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(
            9999999,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        // Act
        let criteria = MatchCriteria::new()
            .set_value(400)
            .set_script(&P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap());
        let matches = tx.match_outputs(&criteria);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], 2);
    }
}
