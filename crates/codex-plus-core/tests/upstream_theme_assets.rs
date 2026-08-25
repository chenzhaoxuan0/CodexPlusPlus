use sha2::{Digest, Sha256};

fn assert_sha256(relative_path: &str, expected: &str) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative_path);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "failed to read upstream theme asset {}: {error}",
            path.display()
        )
    });
    let actual = format!("{:X}", Sha256::digest(bytes));
    assert_eq!(actual, expected, "upstream asset changed: {relative_path}");
}

#[test]
fn bundled_target_renderers_and_styles_remain_byte_exact() {
    for (path, hash) in [
        (
            "assets/inject/upstream/dream-skin/windows/renderer-inject.js",
            "EBB8EAB63ABF129980AD91B2103177A8E0DBA92576E96AE139E5AD8EC542ED6C",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/dream-skin.css",
            "AF3BD8820FF21AA8E1246375150AAEB67EA8161B99067C1787D2C7C9D8324C35",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/theme.json",
            "9068F781F190D213FE5BE180A12AB1ED534FAFDD0BFBA00276C951A178DFE72A",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/renderer-inject.js",
            "EBB8EAB63ABF129980AD91B2103177A8E0DBA92576E96AE139E5AD8EC542ED6C",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/dream-skin.css",
            "AF3BD8820FF21AA8E1246375150AAEB67EA8161B99067C1787D2C7C9D8324C35",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/theme.json",
            "FCCE3F314500BE1A58381FFE7F9A6B212912D54FF9A0E7D17B6664C1516F750E",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/renderer-inject.js",
            "CCA3A09B3E46AAF538CB121ABE7E6D43B6663F9BCEAD090767F55C2EE1D96C62",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/dream-skin.css",
            "0C371B7D794C4783648D1733661E8FA8674C872296CE5CF9898B28EB1765425C",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/renderer-inject.js",
            "19202C8A37C7512E65F950A5516A314867FDF305B74B313F0ABCEA8CF7347F59",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/dream-skin.css",
            "45506CA7C71D4E9867287AE2358C4380C0993F0D04039C29FEE6DBEE20495148",
        ),
        (
            "assets/inject/upstream/snow-skin/renderer-inject.js",
            "9AE8123B51917975B5D4B91995173A6A4DD3C27C6BD5B465B5670C2C1330955A",
        ),
        (
            "assets/inject/upstream/snow-skin/dream-skin.css",
            "97807DE20E40680471D211466B657867CB46280F393EF9D7FBBA5CE829AE5599",
        ),
        (
            "assets/inject/upstream/glass-vision/renderer-inject.js",
            "57A529C0F5743CC7068B5F9064AAB098137520A051E5B0C5A45AD2DFAB91E98C",
        ),
        (
            "assets/inject/upstream/glass-vision/glass-vision.css",
            "84D4AF19D9D5B7D5934139892F83CDB58B5EB370598D775A54587C285A2C8BC1",
        ),
    ] {
        assert_sha256(path, hash);
    }
}

#[test]
fn bundled_skin_pack_theme_files_remain_byte_exact() {
    for (path, hash) in [
        (
            "caishen-lite",
            "68F6AA3C9C68D18014D51E7076A71D9B3F5CA156F339CE3F001F394F0217F941",
        ),
        (
            "caishen-max",
            "02D886D75F779E30E05EB6D6CABC68A9A07EB94B9BCCB4561B82A511DE14F31D",
        ),
        (
            "caishen-readable",
            "5E9947AF7AA00A5CC871330AD55CD9694E49AF54F265C622293C386519F570CB",
        ),
        (
            "export-night",
            "CB07ADE8952BC809497F78F2E73CC886F43F57E4866FC265B7D4E788D63AEE74",
        ),
        (
            "global-founder-bright",
            "6ED25E22A5D9229AD7C7DED3B71EC818D70B0DAE45A650401B8F902F8FA367B9",
        ),
        (
            "mythic-guardian-noir",
            "F4D30003D0F2346C49CECD6398072DB1FEA78ADB8335844DA6ADABE0DDEBA417",
        ),
    ] {
        assert_sha256(
            &format!("assets/inject/upstream/skin-packs/packs/{path}/theme.json"),
            hash,
        );
    }
}
