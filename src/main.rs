use std::collections::VecDeque;

use mini_bitoin::{
    crypto::{
        crypto_utils::{decode_base58, encode_base58},
        hash_helper::hash256,
        private_key::PrivateKey,
        s256_point::{G, S256Point},
        signature::Signature,
    },
    script::{
        helper::p2kh_script,
        op_codes::{self, OP_CODES},
        script::{Script, ScriptCmd},
    },
    transactions::{
        tx::{SIGHASH_ALL, Tx},
        tx_fetcher::TxFetcher,
        tx_in::TxIn,
        tx_out::TxOut,
    },
    utils::errors::BTCErr,
};
use num_bigint::BigInt;
// use mini_bitoin::{
//     crypto::{
//         crypto_utils::little_endian_to_int,
//         hash_helper::{hash256, sha256},
//         private_key::PrivateKey,
//     },
//     script::op_codes::OP_CODES,
//     transactions::{helper::encode_variant, tx::Tx, tx_in::TxIn, tx_out::TxOut},
//     utils::errors::BTCErr,
// };
// use num_bigint::BigInt;

fn main() -> Result<(), BTCErr> {
    // // let passphrase = b"jimmy@programmingblockchain.com my secret";
    // //     // let secret = little_endian_to_int(&hash256(passphrase));
    // //     // let privk = PrivateKey::new(secret);
    // //     // println!("{}",privk.clone().point.address(false, true));
    // //     // // println!("{}",privk.wif(false, true));

    // let _tx = Tx::new(1, false, VecDeque::new(), VecDeque::new(), vec![], 123, true);
    let _hex_tx_string = "0100000000010632e0649f7d5c84b2fa9d3d904f43fcbcd4f202732d2e7be30780c17d336efcb40700000000ffffffff32e0649f7d5c84b2fa9d3d904f43fcbcd4f202732d2e7be30780c17d336efcb40600000000ffffffff32e0649f7d5c84b2fa9d3d904f43fcbcd4f202732d2e7be30780c17d336efcb40b00000000ffffffff32e0649f7d5c84b2fa9d3d904f43fcbcd4f202732d2e7be30780c17d336efcb40100000000ffffffff32e0649f7d5c84b2fa9d3d904f43fcbcd4f202732d2e7be30780c17d336efcb40300000000ffffffff32e0649f7d5c84b2fa9d3d904f43fcbcd4f202732d2e7be30780c17d336efcb40900000000ffffffff07804f12000000000016001437c88fc9329b5a1a149a5f1f81e1acacc1ff598a804f120000000000160014ffeb028a35a00a36b5cf6965d3a9bee40bbad158804f120000000000160014b26f16fafc0909f152207885ad5ce5d62a52329b814f120000000000160014192649765b8680d9cf57ef49654012f76c2c7623814f12000000000016001475bde702b10d0d01e4d80dc83894c44a7964b994814f1200000000001600144aed2473291e28026f140791e3856249cb237ee2c32d0c00000000001600140c87992a3a67474806a718638103e4408625308502473044022067f11155f65c00b76d7e61f527023dfe795bf1878c8f7fceeb3f047f5456f71902201a0412bfa506faac2125797e190aa6e40d7faced2aca879eeafb8a36a2a683b1012103e424f3ff1f42bffb65234cae0bd3f200fa7c8f8ca98a9c21feb5607dc18feb0502483045022100f1c54cd1e72c59dd0c9985581a58312aed60cd90b838e354d965e725a2d38465022036c88cc806b2dd5102761b0d65629686fb7bd8f7178843aea383601dff4a0ae30121023cdcc717aa5f018dc2ac95e16a6117fd507efa7b07f00c2ca2d9f7ed7caf88780247304402205369838c9a21e526727a9a78efd443e20cce8a1dd12bfb48b8ea968063949bb402206656d80fc57df09d8ecc747169c65ea6aff5de82da6c9fb4bd06fa3eb04bf5b0012103e2dff82e416fb86feda4f86a25aaa5a1c0943187c4996f508277df016e2c060802483045022100a790c73f746bf8765b0977fb810d37ab13f55668ae0693c4ffeacd36e71e4a4102200834a4d0bedf0933ae6f0866a13e9b3529605b92547223d9c07153de98db615e0121023a07373be3117d855bf7bb4e70ef1ab1835e18d077303f779e9fbe3de182f6540247304402202d10b6a0deda813058fa75f1b99600297af60f90bed898d75c5897771c626725022048557d0801852ef77e339126fbb0b1fd436d6de616895553b4db3f054f4863e50121030cc944e1b63178d78eac28c6acac2d74614f9d9021e923e0c5b13276ac91a64602473044022056082c9c6cd42d18b836f4fade0aebb56d20d98760ced10df8a1686f0245c0640220235efd6a66b086ab6bdd78f2ab7dec331671be9d8ce1e157fed6ce520154923a012102b6364d67c468b45f16c16c01f882b9472e6deae190bbb17f6fa485c50d5a60df00000000".to_string();
    // let _hex_tx = hex::decode(_hex_tx_string.clone())?;
    // println!("{:?}\n", _hex_tx);
    // let _tx = Tx::parse(_hex_tx_string)?;
    // // println!("{:?}",_tx);
    // let _tx_serialize = _tx.serailize()?;
    // println!("{:?}",_tx_serialize);
    // println!("{:?}\n",_hex_tx==_tx_serialize);
    // let fee = _tx.fee(
    //     "34591e0ab8cf5a0d4bd4f00cf827daf3875179daf97aba21d5e5a04947716265".to_string(),
    //     true,
    // )?;
    // println!("fee: {}", fee);
    // println!(
    //     "{:?}",
    // );
    println!("\n");
    let _hex_tx =
        "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d10000000\
    06b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02\
    207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631\
    e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef0100000000\
    1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc\
    762dd5423e332166702cb75f40df79fea1288ac19430600"
            .to_string();
    // let _tx = Tx::parse(_hex_tx.clone())?;
    // println!("{:?}", hex::decode(_hex_tx.clone())?);
    // // // println!("");
    // println!("{:?}",_tx);
    // // println!("{:?}", _tx.serailize()?);

    // println!(
    //     "are both equ: {}",
    //     hex::decode(_hex_tx.clone())? == _tx.serailize()?
    // );

    // // println!("{:?}",_tx.tx_outs[0].serialize());
    // // println!("{:#?}",_tx);
    // // println!("\n\n\n");
    // let _hex_tx = "\
    // 010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e0100\
    // 00006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951\
    // c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0\
    // da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4\
    // 038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a473044022078\
    // 99531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b84\
    // 61cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba\
    // 1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c35\
    // 6efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da\
    // 6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c3\
    // 4210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49\
    // abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd\
    // 04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea833\
    // 1ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c\
    // 2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20df\
    // e7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948\
    // a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46\
    // 430600"
    //     .to_string();

    // // _tx.parse("0100000004".to_string())?;
    // // _tx.parse("0100000064".to_string())?;
    // // _tx.parse("01000000fd2b02".to_string())?;
    // // _tx.parse("01000000fe7f110100".to_string())?;
    // // _tx.parse("01000000ff6dc7ed3e60100000".to_string())?;
    // // let _tx = _tx.parse(_hex_tx)?;
    // // println!("{:#?}",_tx);
    // // println!("{:?}", BigInt::parse_bytes(b"fd", 16));
    // // println!("{:?}", BigInt::parse_bytes(b"10000", 16));
    // // println!("{:?}", BigInt::parse_bytes(b"100000000", 16));
    // // println!("{:?}", BigInt::parse_bytes(b"10000000000000000", 16));
    // // println!("{:?}",BigInt::from(0x100000000 as u64));
    // // println!("{}", BigInt::from(0x10000000000000000 as u128));
    // // println!("{:?}", encode_variant(BigInt::from(252)));
    // // println!("{:?}", encode_variant(BigInt::from(65534)));
    // // println!("{:?}", encode_variant(BigInt::from(4294967291 as u64)));
    // // println!(
    // //     "{:?}",
    // //     encode_variant(BigInt::from(18446744073709551611 as u128))
    // // );

    // // let mut stack = vec![];
    // // OP_CODES::push_to_stack(OP_CODES::OP_0, &mut stack);
    // // println!("{:?}", stack);
    // // OP_CODES::push_to_stack(OP_CODES::OP_5, &mut stack);
    // // println!("{:?}", stack);
    // // OP_CODES::push_to_stack(OP_CODES::OP_11, &mut stack);
    // // println!("{:?}", stack);
    // // OP_CODES::push_to_stack(OP_CODES::OP_DUP, &mut stack);
    // // println!("{:?}", stack);
    // // OP_CODES::push_to_stack(OP_CODES::OP_HASH256, &mut stack);
    // // println!("{:?}", stack);
    // // OP_CODES::push_to_stack(OP_CODES::OP_HASH160, &mut stack);
    // // println!("{:?}", stack);

    // // let script_pubk1 = hex::decode("1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac").unwrap();
    // // let script_pubk2 = hex::decode("1976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac").unwrap();

    // // println!("scpk1: {:?}, size: {}", script_pubk1, script_pubk1.len());
    // // // println!("scpk1 parse: {:?}")
    // // let out = Script::parse(script_pubk1)?;
    // // println!("{:?}", out);
    // // println!("\n------------------\n");
    // // println!("scpk2: {:?}, size: {}", script_pubk2, script_pubk2.len());
    // // let out = Script::parse(script_pubk2)?;
    // // println!("{:?}", out);

    // // //     ////////////// page 115. Script Evaluation /////////////////////////////
    // let z = BigInt::parse_bytes(b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d", 16);
    //     let sec = hex::decode(String::from("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e\
    // 4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34"))?;
    //     // println!("sec: {:?}", sec);
    //     let sig = hex::decode(String::from("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f4048\
    // 76325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fd\
    // dbdce6feab601"))?;
    //     // println!("sig: {:?}",sig);

    //     let script_pubkey= Script::new(Some(vec![ScriptCmd::Data(sec), ScriptCmd::Op(0xac)]));
    //     let script_sig = Script::new(Some(vec![ScriptCmd::Data(sig)]));
    //     // println!("{:?}",script_pubkey + script_sig);
    //     let combine_script = script_sig + script_pubkey;
    //     println!("{}", combine_script.evaluate(z));

    //////////////////// Tx Fetcher////////////////////
    //
    // let tx_fetcher = TxFetcher::new();
    // tx_fetcher.fetch(
    //     "34591e0ab8cf5a0d4bd4f00cf827daf3875179daf97aba21d5e5a04947716265".into(),
    //     true,
    //     true,
    // )?;

    //     //  ////////////////////// Check or Fee //////////////////////
    //     let raw_tx = String::from(
    //         "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf830\
    // 3c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccf\
    // cf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8\
    // e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278\
    // afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88a\
    // c99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600",
    //     );
    //     let tx = Tx::parse(raw_tx)?;
    //     // println!("{:?}", tx);
    //     let fee = tx.fee(false)?;
    //     println!("fee: {}", fee);

    //     // ///////////////////// script eval /////////////////////
    //     let z = BigInt::parse_bytes(
    //         b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
    //         16,
    //     );
    //     let mut sec = hex::decode(
    //         "04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e\
    // 4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
    //     )?;
    //     sec.push(0xac);

    //     let sig = hex::decode(
    //         "3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f4048\
    // 76325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fd\
    // dbdce6feab601",
    //     )?;

    //     let script_pubkey = Script::new(Some(vec![ScriptCmd::Data(sec), ScriptCmd::Op(0xac)]));
    //     let script_sig = Script::new(Some(vec![ScriptCmd::Data(sig)]));

    //     // println!("{:?}", script_pubkey);
    //     // println!("{:?}", script_sig);
    //     let combined_script = script_sig + script_pubkey;
    //     // println!("combined script: {:?}", combined_script);
    //     println!("eval: {:?}", combined_script.evaluate(z));

    // println!("{}", op_codes::OP_CODES::OP_13 as u32);
    // // println!("{:?}",);
    // OP_CODES::stack_operations(172.into(), &mut VecDeque::new(), &mut VecDeque::new(), z);

    //     ////////////////////////////////////// check fee /////////////////////////////////////////////
    //     let raw_tx = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf830\
    // 3c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccf\
    // cf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8\
    // e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278\
    // afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88a\
    // c99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600".to_string();
    //     let tx = Tx::parse(raw_tx)?;
    //     dbg!(&tx);

    //     println!("tx fee: {}", tx.fee(false)? >= 0); // if fee is negative then the output_sum is greater than input_sum

    // ////////////////////////////////////// verify signature /////////////////////////////////////////
    // let sec = hex::decode("0349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a")?;
    // let der = hex::decode("3045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed")?;
    // let z = BigInt::parse_bytes(b"27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6", 16).unwrap();
    // let point = S256Point::parse(sec);
    // let signature = Signature::parse(der);
    // let generator = G.clone();
    // println!("Verify sig: {}", point.verify(z, signature, generator));

    // // 1.1 checking signature
    // let _hex_tx =
    //     "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d10000000\
    // 06b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02\
    // 207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631\
    // e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef0100000000\
    // 1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc\
    // 762dd5423e332166702cb75f40df79fea1288ac19430600"
    //         .to_string();
    // let mut _tx = Tx::parse(_hex_tx.clone(), false)?;
    // // println!("{:?}",_tx);
    // // let nz = _tx.sig_hash(None)?;
    // println!("{}", _tx.verify()?);
    // // println!("{}", _tx.verify_fast()?);
    // // println!("{:?}", BigInt::parse_bytes(nz.as_bytes(), 16));
    // // dbg!(_tx);
    // // println!("tx in len: {}", _tx.tx_ins.len());
    // // _tx.tx_ins.get(0).unwrap().
    // // println!("{:?}", hex::decode(_hex_tx.clone())?);
    // // // // println!("");
    // // println!("{:?}",_tx);

    // // println!("{:?}",1_u32.to_le_bytes());

    // ////////////////////////////////////// encode 58 and decode 58 /////////////////////////////////////////
    // println!("{:?}", encode_base58(&[0,0,0,40,127,180,205]));
    // println!("{:?}", decode_base58("111233QC4")?);
    // // println!("{:?}", decode_base58("NTRAJ9EEjHFHhHZvMKEKfkceg5V9ppx5ZP"));

    // let b= "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    //     .chars()
    //     .collect::<Vec<_>>();

    // ////////////////////////////////////// create transaction /////////////////////////////////////////
    // let prev_tx = hex::decode("0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299")?;
    // let prev_ind = 13;
    // let tx_in = TxIn::new(prev_tx, prev_ind, None, 0xfffffffe);
    // let mut tx_outs = VecDeque::new();
    // let change_amt = (0.33*100000000.0) as u64;
    // let change_h160 = decode_base58("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2")?;
    // let change_script = p2kh_script(change_h160)?;
    // let change_output = TxOut::new(change_amt, change_script);

    // let target_amt = (0.1*100000000.0) as u64;
    // let target_h160 = decode_base58("mnrVtF8DWjMu839VW3rBfgYaAfKk8983Xf")?;
    // let target_script = p2kh_script(target_h160)?;
    // let target_output = TxOut::new(target_amt, target_script);
    // tx_outs.extend([change_output,  target_output]);
    // let mut tx_obj = Tx::new(1, false, vec![tx_in].into(), tx_outs, vec![], 0, true);
    // // println!("{:#?}",tx_obj);

    // //////////////////////////////////////// Sign Transaction ///////////////////////////////////////////
    let mut transaction = Tx::parse("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600".to_string(), false)?;
    // Script::')
    println!("{}", transaction.fee(false)?);
    // let z_str = transaction.sig_hash(Some(0))?;
    // let z = BigInt::parse_bytes(z_str.as_bytes(), 16);
    let z = BigInt::parse_bytes(b"5577958942132845675238362358398520344146083469766013512921123674672842307630", 16);
    println!("{:?}",z);
    let priv_key = PrivateKey::new(BigInt::from(8675309));
    let mut der = priv_key.sign(z.unwrap()).der();
    // let mut der = priv_key.sign(BigInt::parse_bytes(b"86ec9e55e3658d0246f188e2c52bfec8f85e6ed25ae8b3663acd702b36c7ff45", 16).unwrap()).der();
    // // println!("{:?}", BigInt::parse_bytes(b"86ec9e55e3658d0246f188e2c52bfec8f85e6ed25ae8b3663acd702b36c7ff45", 16));
    der.extend(SIGHASH_ALL.to_be_bytes());
    let sig = der;
    let sec = priv_key.point.sec(false);
    let script_sig = Script::new(Some(vec![ScriptCmd::Data(sig), ScriptCmd::Data(sec)]));
    if let Some(tx_in) = transaction.tx_ins.get_mut(0) {
        tx_in.script_sig = script_sig;
    }
    let tx_obj_serial = transaction.serialize()?;
    // println!("{:?}",tx_obj_serial);
    let s = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006a47304402207db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11022010178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67feffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    println!("{:?}", hex::decode(transaction.serialize()?));
    // println!("{:?}", hex::decode(s)?);
    Ok(())
}
