#[cfg(test)]
mod tests;

use ic_ethereum_types::Address;

macro_rules! ethereum_address {
    ($address:expr) => {
        Address::new(hex_literal::hex!($address))
    };
}

/// The list of addresses to which we do not allow retrievals.
/// This list has been downloaded from:
/// https://www.treasury.gov/ofac/downloads/prgrmlst.txt
/// The script used to construct this list is available:
/// /rs/ethereum/cketh/minter/scripts/ofaq_blocklist.py
/// NOTE: Keep it sorted!
const ETH_ADDRESS_BLOCKLIST: &[Address] = &[
    ethereum_address!("01e2919679362dFBC9ee1644Ba9C6da6D6245BB1"),
    ethereum_address!("03893a7c7463AE47D46bc7f091665f1893656003"),
    ethereum_address!("04DBA1194ee10112fE6C3207C0687DEf0e78baCf"),
    ethereum_address!("05E0b5B40B7b66098C2161A5EE11C5740A3A7C45"),
    ethereum_address!("07687e702b410Fa43f4cB4Af7FA097918ffD2730"),
    ethereum_address!("0836222F2B2B24A3F36f98668Ed8F0B38D1a872f"),
    ethereum_address!("08b2eFdcdB8822EfE5ad0Eae55517cf5DC544251"),
    ethereum_address!("09193888b3f38C82dEdfda55259A82C0E7De875E"),
    ethereum_address!("098B716B8Aaf21512996dC57EB0615e2383E2f96"),
    ethereum_address!("0E3A09dDA6B20aFbB34aC7cD4A6881493f3E7bf7"),
    ethereum_address!("0Ee5067b06776A89CcC7dC8Ee369984AD7Db5e06"),
    ethereum_address!("12D66f87A04A9E220743712cE6d9bB1B5616B8Fc"),
    ethereum_address!("1356c899D8C9467C7f71C195612F8A395aBf2f0a"),
    ethereum_address!("169AD27A470D064DEDE56a2D3ff727986b15D52B"),
    ethereum_address!("178169B423a011fff22B9e3F3abeA13414dDD0F1"),
    ethereum_address!("179f48C78f57A3A78f0608cC9197B8972921d1D2"),
    ethereum_address!("19Aa5Fe80D33a56D56c78e82eA5E50E5d80b4Dff"),
    ethereum_address!("1da5821544e25c636c1417Ba96Ade4Cf6D2f9B5A"),
    ethereum_address!("1E34A77868E19A6647b1f2F47B51ed72dEDE95DD"),
    ethereum_address!("22aaA7720ddd5388A3c0A3333430953C68f1849b"),
    ethereum_address!("23173fE8b96A4Ad8d2E17fB83EA5dcccdCa1Ae52"),
    ethereum_address!("23773E65ed146A459791799d01336DB287f25334"),
    ethereum_address!("242654336ca2205714071898f67E254EB49ACdCe"),
    ethereum_address!("2573BAc39EBe2901B4389CD468F2872cF7767FAF"),
    ethereum_address!("26903a5a198D571422b2b4EA08b56a37cbD68c89"),
    ethereum_address!("2717c5e28cf931547B621a5dddb772Ab6A35B701"),
    ethereum_address!("2f389cE8bD8ff92De3402FFCe4691d17fC4f6535"),
    ethereum_address!("2F50508a8a3D323B91336FA3eA6ae50E55f32185"),
    ethereum_address!("2FC93484614a34f26F7970CBB94615bA109BB4bf"),
    ethereum_address!("330bdFADE01eE9bF63C209Ee33102DD334618e0a"),
    ethereum_address!("35fB6f6DB4fb05e6A4cE86f2C93691425626d4b1"),
    ethereum_address!("3aac1cC67c2ec5Db4eA850957b967Ba153aD6279"),
    ethereum_address!("3Cffd56B47B7b41c56258D9C7731ABaDc360E073"),
    ethereum_address!("3e37627dEAA754090fBFbb8bd226c1CE66D255e9"),
    ethereum_address!("3eFA30704D2b8BBAc821307230376556cF8CC39e"),
    ethereum_address!("407CcEeaA7c95d2FE2250Bf9F2c105aA7AAFB512"),
    ethereum_address!("4736dCf1b7A3d580672CcE6E7c65cd5cc9cFBa9D"),
    ethereum_address!("47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936"),
    ethereum_address!("48549A34AE37b12F6a30566245176994e17C6b4A"),
    ethereum_address!("502371699497d08D5339c870851898D6D72521Dd"),
    ethereum_address!("527653eA119F3E6a1F5BD18fbF4714081D7B31ce"),
    ethereum_address!("538Ab61E8A9fc1b2f93b3dd9011d662d89bE6FE6"),
    ethereum_address!("53b6936513e738f44FB50d2b9476730C0Ab3Bfc1"),
    ethereum_address!("5512d943eD1f7c8a43F3435C85F7aB68b30121b0"),
    ethereum_address!("57b2B8c82F065de8Ef5573f9730fC1449B403C9f"),
    ethereum_address!("58E8dCC13BE9780fC42E8723D8EaD4CF46943dF2"),
    ethereum_address!("5A14E72060c11313E38738009254a90968F58f51"),
    ethereum_address!("5cab7692D4E94096462119ab7bF57319726Eed2A"),
    ethereum_address!("5efda50f22d34F262c29268506C5Fa42cB56A1Ce"),
    ethereum_address!("5f6c97C6AD7bdd0AE7E0Dd4ca33A4ED3fDabD4D7"),
    ethereum_address!("610B717796ad172B316836AC95a2ffad065CeaB4"),
    ethereum_address!("653477c392c16b0765603074f157314Cc4f40c32"),
    ethereum_address!("67d40EE1A85bf4a4Bb7Ffae16De985e8427B6b45"),
    ethereum_address!("6aCDFBA02D390b97Ac2b2d42A63E85293BCc160e"),
    ethereum_address!("6Bf694a291DF3FeC1f7e69701E3ab6c592435Ae7"),
    ethereum_address!("6F1cA141A28907F78Ebaa64fb83A9088b02A8352"),
    ethereum_address!("722122dF12D4e14e13Ac3b6895a86e84145b6967"),
    ethereum_address!("723B78e67497E85279CB204544566F4dC5d2acA0"),
    ethereum_address!("72a5843cc08275C8171E582972Aa4fDa8C397B2A"),
    ethereum_address!("743494b60097A2230018079c02fe21a7B687EAA5"),
    ethereum_address!("746Aebc06D2aE31B71ac51429A19D54E797878E9"),
    ethereum_address!("756C4628E57F7e7f8a459EC2752968360Cf4D1AA"),
    ethereum_address!("76D85B4C0Fc497EeCc38902397aC608000A06607"),
    ethereum_address!("776198CCF446DFa168347089d7338879273172cF"),
    ethereum_address!("77777FeDdddFfC19Ff86DB637967013e6C6A116C"),
    ethereum_address!("7Db418b5D567A4e0E8c59Ad71BE1FcE48f3E6107"),
    ethereum_address!("8281Aa6795aDE17C8973e1aedcA380258Bc124F9"),
    ethereum_address!("833481186f16Cece3f1Eeea1a694c42034c3a0dB"),
    ethereum_address!("83E5bC4Ffa856BB84Bb88581f5Dd62A433A25e0D"),
    ethereum_address!("84443CFd09A48AF6eF360C6976C5392aC5023a1F"),
    ethereum_address!("88fd245fEdeC4A936e700f9173454D1931B4C307"),
    ethereum_address!("910Cbd523D972eb0a6f4cAe4618aD62622b39DbF"),
    ethereum_address!("94A1B5CdB22c43faab4AbEb5c74999895464Ddaf"),
    ethereum_address!("94Be88213a387E992Dd87DE56950a9aef34b9448"),
    ethereum_address!("94C92F096437ab9958fC0A37F09348f30389Ae79"),
    ethereum_address!("9AD122c22B14202B4490eDAf288FDb3C7cb3ff5E"),
    ethereum_address!("a0e1c89Ef1a489c9C7dE96311eD5Ce5D32c20E4B"),
    ethereum_address!("A160cdAB225685dA1d56aa342Ad8841c3b53f291"),
    ethereum_address!("a5C2254e4253490C54cef0a4347fddb8f75A4998"),
    ethereum_address!("A60C772958a3eD56c1F15dD055bA37AC8e523a0D"),
    ethereum_address!("aEaaC358560e11f52454D997AAFF2c5731B6f8a6"),
    ethereum_address!("af4c0B70B2Ea9FB7487C7CbB37aDa259579fe040"),
    ethereum_address!("af8d1839c3c67cf571aa74B5c12398d4901147B3"),
    ethereum_address!("b04E030140b30C27bcdfaafFFA98C57d80eDa7B4"),
    ethereum_address!("b1C8094B234DcE6e03f10a5b673c1d8C69739A00"),
    ethereum_address!("B20c66C4DE72433F3cE747b58B86830c459CA911"),
    ethereum_address!("b541fc07bC7619fD4062A54d96268525cBC6FfEF"),
    ethereum_address!("BA214C1c1928a32Bffe790263E38B4Af9bFCD659"),
    ethereum_address!("bB93e510BbCD0B7beb5A853875f9eC60275CF498"),
    ethereum_address!("c2a3829F459B3Edd87791c74cD45402BA0a20Be3"),
    ethereum_address!("Ca0840578f57fE71599D29375e16783424023357"),
    ethereum_address!("CC84179FFD19A1627E79F8648d09e095252Bc418"),
    ethereum_address!("CEe71753C9820f063b38FDbE4cFDAf1d3D928A80"),
    ethereum_address!("D0975B32cEa532eaDDdFC9c60481976e39dB3472"),
    ethereum_address!("D21be7248e0197Ee08E0c20D4a96DEBdaC3D20Af"),
    ethereum_address!("d47438C816c9E7f2E2888E060936a499Af9582b3"),
    ethereum_address!("D4B88Df4D29F5CedD6857912842cff3b20C8Cfa3"),
    ethereum_address!("D5d6f8D9e784d0e26222ad3834500801a68D027D"),
    ethereum_address!("D691F27f38B395864Ea86CfC7253969B409c362d"),
    ethereum_address!("D692Fd2D0b2Fbd2e52CFa5B5b9424bC981C30696"),
    ethereum_address!("D82ed8786D7c69DC7e052F7A542AB047971E73d2"),
    ethereum_address!("d8D7DE3349ccaA0Fde6298fe6D7b7d0d34586193"),
    ethereum_address!("d90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b"),
    ethereum_address!("d96f2B1c14Db8458374d9Aca76E26c3D18364307"),
    ethereum_address!("DD4c48C0B24039969fC16D1cdF626eaB821d3384"),
    ethereum_address!("df231d99Ff8b6c6CBF4E9B9a945CBAcEF9339178"),
    ethereum_address!("DF3A408c53E5078af6e8fb2A85088D46Ee09A61b"),
    ethereum_address!("e7aa314c77F4233C18C6CC84384A9247c0cf367B"),
    ethereum_address!("eDC5d01286f99A066559F60a585406f3878a033e"),
    ethereum_address!("f4B067dD14e95Bab89Be928c07Cb22E3c94E0DAA"),
    ethereum_address!("F60dD140cFf0706bAE9Cd734Ac3ae76AD9eBC32A"),
    ethereum_address!("F67721A2D8F736E75a49FdD7FAd2e31D8676542a"),
    ethereum_address!("F7B31119c2682c88d88D455dBb9d5932c65Cf1bE"),
    ethereum_address!("FD8610d20aA15b7B2E3Be39B396a1bC3516c7144"),
    ethereum_address!("ffbaC21a641Dcfe4552920138D90F3638B3c9fba"),
];

pub fn is_blocked(from_address: &Address) -> bool {
    ETH_ADDRESS_BLOCKLIST.binary_search(from_address).is_ok()
}
