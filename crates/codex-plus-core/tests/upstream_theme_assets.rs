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
            "DB48AE78497EB9C1EB800F32C110778490A3913EDD969429697DC3EBD292DE97",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/dream-skin.css",
            "AFF9433B526D7149DA99B7C07367B75BCA558F9D8114243331FB9099B54E2916",
        ),
        (
            "assets/inject/upstream/dream-skin/windows/theme.json",
            "9068F781F190D213FE5BE180A12AB1ED534FAFDD0BFBA00276C951A178DFE72A",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/renderer-inject.js",
            "DB48AE78497EB9C1EB800F32C110778490A3913EDD969429697DC3EBD292DE97",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/dream-skin.css",
            "AFF9433B526D7149DA99B7C07367B75BCA558F9D8114243331FB9099B54E2916",
        ),
        (
            "assets/inject/upstream/dream-skin/macos/theme.json",
            "FCCE3F314500BE1A58381FFE7F9A6B212912D54FF9A0E7D17B6664C1516F750E",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/renderer-inject.js",
            "0BFB5F66A0323BF1392B42033E66904DE3EC4BFC8A5BA297F2BB92A4A6740A34",
        ),
        (
            "assets/inject/upstream/cidala-tiger/windows/dream-skin.css",
            "482A60AF98DD6B460BF624C56918C5B57F9CCD5B55E52FA46D486F7D65259D9A",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/renderer-inject.js",
            "21FAF1DC0A3EBE78D8D972182CACE62BD93D5D0E5841725398A4A524EF2BC20B",
        ),
        (
            "assets/inject/upstream/cidala-tiger/macos/dream-skin.css",
            "5E149E9A13985961C5F3125296178ACB2ABF0B528974F1E616AA625970430562",
        ),
        (
            "assets/inject/upstream/snow-skin/renderer-inject.js",
            "0FCDFF4AECD03EAB2CA4EE923CCD20CB97EB5460F7C9F07351A2003FFA76E6FA",
        ),
        (
            "assets/inject/upstream/snow-skin/dream-skin.css",
            "0AF2D20FBE3E3DD13F0BE7F1E5A90366E1501084827B22C1D4815A421BFCE823",
        ),
        (
            "assets/inject/upstream/glass-vision/renderer-inject.js",
            "D14943E95DB62DB81BF29D9CF14FCAF1DD1EA9A9625245C020865127EEA295A2",
        ),
        (
            "assets/inject/upstream/glass-vision/glass-vision.css",
            "4C37C53544EE4F1CD93BA5D0DC3E174B05D4CB84EC9A436295D11D19F0BB04F1",
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
            "379CB601522E7A5C2FC906E3D8BD5C7C64385FBD2A428798F8D169DEB7026F2E",
        ),
        (
            "caishen-max",
            "0CFF815EC9582B88ECF4AD9E7562D7DFF32904FA8F09338811397960D62FD7D4",
        ),
        (
            "caishen-readable",
            "EB1FFD4F2F49137B4AEDDBED435513D42685C8ED9E97DF644C693FD7859CC62D",
        ),
        (
            "export-night",
            "C312329AABEE84B9A8443B08D4DB64863EC49DEEA3C7F7C942B57E4391B87B59",
        ),
        (
            "global-founder-bright",
            "EAFB018494225ABABD83AE0E7B940E3F565232CF8F30C53AAFA63E7652178810",
        ),
        (
            "mythic-guardian-noir",
            "2A57716D0161F7405D713912BCD0CD329038657518537F3EFDB5F7EE53DBAE3D",
        ),
    ] {
        assert_sha256(
            &format!("assets/inject/upstream/skin-packs/packs/{path}/theme.json"),
            hash,
        );
    }
}
