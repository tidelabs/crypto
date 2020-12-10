// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(non_snake_case)]

#[cfg(all(feature = "hmac", feature = "sha"))]
pub fn PBKDF2_HMAC_SHA512(password: &[u8], salt: &[u8], c: usize, dk: &mut [u8; 64]) -> crate::Result<()> {
    if c == 0 {
        return Err(crate::Error::InvalidArgumentError {
            alg: "PBKDF2-HMAC-SHA512",
            expected: "non-zero iteration count",
        });
    }

    pbkdf2::pbkdf2::<hmac_::Hmac<sha2::Sha512>>(password, salt, c as u32, dk);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestVector {
        password: &'static str,
        salt: &'static str,
        c: usize,
        dk: &'static str,
    }

    #[test]
    #[cfg(all(feature = "hmac", feature = "sha"))]
    fn test_PBKDF2_HMAC_SHA512() -> crate::Result<()> {
        let tvs = [
            TestVector {
                password: "",
                salt: "",
                c: 1,
                dk: "6d2ecbbbfb2e6dcd7056faf9af6aa06eae594391db983279a6bf27e0eb2286143ab0c996f33ca4b667e945829ea693340f2831797324e5f31df18ed171d18c97",
            },
            TestVector {
                password: "",
                salt: "",
                c: 58238,
                dk: "cfeaf75b86e09797f7c852e16d5f1c5b65356b6f2e60f403f8a273f2685e0439a7ee09a78d8438fdb9770faab8a68a764faa0e12bb53e3515e4e149836772a15",
            },
            TestVector {
                password: "",
                salt: "980ee79e5dc452dbf876b51823b5b57d",
                c: 91063,
                dk: "3b48784708ed53b94d6da51589fbac81a4abc2262a81e70de4c5e7f5339114b42e163b15b3b5a769a87f8ef3caf27e836ff9f68dd3712885ac0cadb894668dd7",
            },
            TestVector {
                password: "2629d3f2048e26bf741a4540334292fc3be5151c558aea9297e19207eda9acf74f36ce187b357de1ddc1763ad183af5ebb4c1edf24f5b34e6c5845b63e462f1b56251ecf502e0bff873383b6ebe919d15b68019d803a7dcb1d92d85c6c32ee1d9de64f79b3f2e832a0bff473741ac0eb3ca4335cad9f3f5967d47b2f78eb049027ba5af008c5f1f422228845dbd14095c49c27dcffb17578cbb215bf17226efb066ed8f5fd6bcf104f06d609ebcc9441c99de396ba73c9f389fe560726133d0085e595e3d14d85f02b728eb3d4e859dcf0387948ee949806e775920673d89932cc107d5f73efd08c5c8e483f5eacf8f33d3b396ead37be05e8f9e67680ab3489637de3b3e3b879e405fb047b95175f5fe53fe93b75b75ad2a0d7220b8cecc966e2d9d4e6bc2b78ff9653811e461caa452d7b0d70e6186a8efc977929ed816c0188dcc84e6bf316adcf0dead5ec4a4ca1db19",
                salt: "",
                c: 300,
                dk: "904173464bbd6e49d7d6271af423b81dd3ed3042c34c9810c9afb99974ce076a5f3aad865436eb3ec50999ab4fd5af2e6267b4be4821ee42f54ccf57731612b2",
            },
            TestVector {
                password: "5cc8c655c945a06708cfac8c875e6bc83b86c2b75979d4550b27ca149cb0d9e29e869f2113734a9d51398c4f33156c57ded7b28a366f159befedad52ba4d200528bdfc0100f4cdff82dfe6de11",
                salt: "7efcd7ca8191297696c35c756aae0961599fa96276a4b07f05d98b393bc06238dfd0f67580a0ae8fa71ceee2b15d2453cd8f100c54e2a81d31e47e00b9c0be2d498b0acac4a0fbea861bca40e938eb8b84c2c4c529bb1354aea6e1ae209d2057e58d28bf2d1ac2cf38853f67175bfebace297f30eaf7458477a21e07e2c6d2643bf64971b54c51a0656eaaa8ffae7706adff1b5eb8eb0d433095f244a2964c0de049aa18dc74e3c38a3033974b48787cafe3c0869c29cffa0eb1f612c3088f71e01585cced1613135923b79b0f821001417067862f336f28ad111ad023707692607be4d5ba2f7e3a9e44822df1b564a927922f68a7cb3b7bb4d33b9844acb37eb28c6a302f35e1d6fb0e420c35ea189f55459482f92d94856bf1a9e27ed3ec69a75c7ab41d7de5babc13e772e859202fb1f2bdb372108151ec400926ace505421bc12c5e941132e67ffd2bc940f33ee86a28881420f5eef3c24de1af45413cf8470fc97a959d4a8f8d84918e249666bea7d8c0f37c2c59004bd0065b8e7c679b36cec183c1c6181ae9d500bb79713a7b2a74347114169a69bc50f1bf597cef9df66054a6e22a28f0921983e2bc61f5b2ce890b5892fe10848767b30ffb0b7c3e85bc7995ad16d99d6ed3412c197c88563b80627fbf8dba11806560f0510b5d0b696983ff954883566aeb716b099cc26fa95c9a6839d6d8500d0ed30d0a9667065433df5f881d7e80357e2e659215e8ad2b017a9b73cf5b559f291be1bb0583c3b99121d222c37941b2a458d4d95756f4b73337335568ed2ed1c2905f72bc24fd4be5bf097889aa6a20e3407f7d0367d9f308175914fac586bcba893af1f2f08a4f76061e29b0c8af8e8b94de546593ceab57dab88938dd93aa8847049e43b644f13e5e1b1a9a00eff393d0a9f6860fc3b2d5df23d95f68c862571ef409fc8be5007a78c5206b5a400d9f45d52a959cad5cd4c711b5783ccf62e626166d7da95b15a463d3e13ed6bf580709b402b1378715f4b99bceed9a67f04e0d92e6d10112d32e7470433e60d6f99670bec25d70fc65f9a19104125b14ca99102ff28ac54e27d43062c301249bc5f2885ffd5c3c5655c0cd4f947bd95749f869ddc711fb402e368b94a90445a28f5c57e2c34830d9a4e28c8f3220ceb5ad4dee551ae52ff36558ed618bf945f4164ea67e57e73f34f19dbe",
                c: 17742,
                dk: "766c06d5bcafa0e93236fcdf01b2d5774a72e139c3d7c2c24d56584f94d7927eea00339ff9a13f518522b379d45030bf75aa98d09b390a146d11fe6add227365",
            },
            TestVector {
                password: "fbb046bc8014b57a659b732110ce3713c1dded417f967744c5aa3895cbf26f18e285357db3524403d57d64a9f15ac66d5de5974b8f1b08c6b6967b9b1830fec8be90d7311967a79b5c81eb539fc2b8029e5428adea1f64b351a1eca102442f81932fabab475aabb54c",
                salt: "30fcc1233573b067014a851cc5aacc12435675116dd3ff17027f573182116548be4d005755814bf71e98797609423373339799c500b29ee9d40f916aac2ca95003b403ce839fbe0efe456eaf74eb63d562e9d848fd40ac642a9eba37010bf3e51734506f4162ffd9afdce382adb3d33cda907121c65eca3e5b355f5547c045b7cab689dfec5aca01a44d91b80ae4132ea5e90413da5f808d19519df107c27f2c0bdc4c622be294aac5be54971bdd7ed44b9f746a283b924c14c8b69375aa10e2340557fa3e5a682026b277c4828dc308b1c2b9ec59d857ed4d28d48e8616509c6c44eebdcdd4ac8faf6d5f16854df38380627c2156ecafe14058e2ad89c4a5d767a85dbbba5cab44e3b17982185c49792f7b48189346a10c9ed268093b7917f3ff62075274e207cd1395f17b5f672d590a245d98392cd605864706095827feb840f87669f64263e2f03dad1586dcad334d652f6e97c3fa4905ded6c87baad9bca2052eafc181fd01b8f90b5d934fc73ab8de45e7d2698faa898f34b1e2517662ed02cd2bf8bf87d85498c7591e589a613e96304d26173a70fdea1d90f9a3875d8f4d7299b91cd873ed006606187beb4cba595a7bcc537356136a68765a8bf69c3edd682cdd9469e07e7ed9a0c94294569cb365fc7058df5dc72fe9dbbeb06e1a4845283ecf19490712dbdd54379478362a60d21a66e762d11f3fa9907b8b70e2a02b3e82e3e2d432e200a564c5ae5dc3da7568cc6896b3f9c7890825ef04297ca31ab937d7eb5b3d872da67f6b71dbe7f064229c94e5bab5ac708577cb412d6c6cdd3854a884d4ad8274f78f8b4358c2b75c2117a6fa36105ded4cf29f2ffc8e860ef03ec5b16c648e9eb3411ebcbdd9421336f06ac7bcfa7b93b1d8b1658e3cb8b74f1f97e3861a73a67c9d1091d05fa28cb293309ad61af4010cd3677bc74764c18b4ebc8554110ab6e412b8c5e5574aa247bc3c3ca708a449df41bfd30a061e371a387c1135ce51915d78119439df748d126f1249f499004b405f32a3deae7bdfd2bc605781f889ced56e8070e1ed730bfbc161347c3771da36fa0b1e86de73ecac1bdf667823148fd43e4d05fd3d9c27106796a402c4cc3fa785fa9e804a8dcfbe3627fc2e2f5ddb765db92a7f8c7a7c7e4b10b6d3a1b1d416d81ed6b77776c260dab315e49a66eb84dc3141dc2ce40f221d57def99cf9f63023a92e4c97dd793bd0656c555ec5b84a992af4871d37d2d6b73da4b1ca6fa80522a7137d83f0e2d1b00d3ba9be723161bfa0b0e225b11b90447a120d5b237fea8e916ebf3a74e2f03d29230dd52ea257ed0acf7cf461c1fdeb",
                c: 8217,
                dk: "2ffe113d42cfa90dbfcd68c48fa2067ef0417eb36c0d1f01040d6a31dee35686ffcd5eca783ad16124c9fb3743cc9eea7d67166aa58837e7be23c3f0fee85439",
            },
            TestVector {
                password: "f6050aedc16aae99aad2f96844f1ab38db58e80508a08ff769ab014c15fe21a85a441695fc0c220dadd20eecd73fa82e0c6c37ff72aa15245d5e8663cd953e9ee99fb9dc53d9108e44cb2d87cedb9cd9415c423284b36f4973510eb586e3c6687438039ef2500781b797d005f2556f08f15dbe60be387638e17f186bc6e8239c59c4d118a3aa0031935755f333b02450265aaa8335f89903580da6b7bb8509d2ee24a7fbc71bb6f8a9baefb9",
                salt: "b679d39b5ec4a01ccf088cb488ebd8f135a99e3a119ffcf85784df7f7391005e547bdd05b6bc500273fdbc9f1eae3ba6ef43be11c9690a5633934c8a2c4e355ad9a2dc4b44cef8184dd1dca031f50e333cf049b2e8a0501a9a59970c0dd2b553659d4f77e974c5747b1799054e1447b859b93f2a19a95263f9d2dbb0e11dee6f6f53bd1ad78854ebec4f7f7785f38006607d9afde79a5dde5b830ea02353a28d9b35c900eae4cb1667ac363787b329028f0ad2bde3cef245008ca8a7619d862d57e61122ccd6c9456d56bb3329dcba776c10dae596b287760e8356fa0123c3bc1211f22edd20c8a829165290bb605cdd6ab1c371e0c76fadce1ea8cc4cd6b5b34af8a8ba9cac45433f4f974d2dabc54ac4797098f9ab4b79690a804a5ce3684d3fc9eb6fa91484783ee47d74754368856b407c78de5bec3d80da92897e4f053c42fab6748dd45d72a751d85f14de12523049da82839553fcc786c5e29a43a87eac1723e55ffa12b8a506215f9a0072e42d972da1ccb2840e608affa32545d051d53259b8f8348a7e0875398852d9da28846e9106929a0851aed063647b4dca44a79eafaf6c1f68d0d215d5fb271942babbba9fd95af89d854c48e6292c6b8cc780ebae5982b5ad60b3a7affd705f7fda8ea38168de383679ba3ccec6ce3b9b9f657f4ce62bad354fc8e8a5e92cd72e18ec1e6bf2386a4b4588d50bf0acaf83066899db83c834017183dc08a7e52b9c7ebb34ba35078924e0fd8831c1b5b2cb1705c2344d9f96129a8c40ba16834313ae5572102e2ba270a1796349138fa1cfa840deddebc043590a78c6fdbd0e83f23d8fb87c1a987c96b47bd3caaa2d8fa8e435bf08b7825a92e8d2c934f6c430c180f8f3f3532e86de2c40204575e86042025176d1e36370ea1c048967808b61cc9ea808ebfdf08e4a1783f3f8aa8b76360279aa724d1cf9aa54b8c36f931c1e17c7f85ca45738faba7567d342e2849fdc2d345439b674a1948f9ea2833e8ae10b41ce7f943bca7d0b2690414637ee6118f6f1f8d068a66831025df71a85c7d66455daabac43e66d8c2b30c240b416692e8ff00f4951d0bfb3d3f64e0af9f4ebeba1154617fe56f49d31cbc9d5b7a34ed7b8f8a31b235bb982dc9b3e0ce6b5b121822e072c31ef1cc1a6bb6403c6f5722cbe322dacca843d35014ed295d0427c7220447d515b88d864e30bb79fe6c0f82d2e7908fb2a2ab9311482aaa60c0d3548",
                c: 75797,
                dk: "30a322524b4cce6d0985197a71268160102faf7ea8911fae5a32c70950fac6258e7ed04140424fea4d96eb8086ae292c62a7e0618a966f28d9e25356096c6171",
            },
        ];

        for tv in tvs.iter() {
            let password = hex::decode(tv.password).unwrap();
            let salt = hex::decode(tv.salt).unwrap();
            let mut expected_dk = [0; 64];
            hex::decode_to_slice(tv.dk, &mut expected_dk).unwrap();

            let mut dk = [0; 64];
            PBKDF2_HMAC_SHA512(&password, &salt, tv.c, &mut dk)?;
            assert_eq!(dk, expected_dk);
        }

        Ok(())
    }
}
