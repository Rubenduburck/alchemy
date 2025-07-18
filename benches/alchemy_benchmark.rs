use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use alchemy::{client::Client, encode::encoding::Encoding};

fn criterion_benchmark(c: &mut Criterion) {
    let client = Client::new();
    const TEST_INT: &str =
        "113040274929467469880280419389222560002940607015822970101697264377219490217140";
    const TEST_HEX: &str = "0x9407f1d4a06a45be932f6cf70f0e16ec5bf6d24d916ee21f33460e6beeef3937";
    const TEST_BASE64: &str = "nZ2tLQvuxccQtgr5Rqh4aAZqzhQG3vKCfdCVVEEWvVg";
    const TEST_VERY_LARGE_HEX: &str = "0x256c725644b473f5d0abab303696d530a20b353251f8404d446385231a75d125f4ff9cf67eb57c3cc046be0b738bdbe961820fcf2259431e0da0c813f89f3ab40465e89791760d74e9d602c006352613a740ed6891dbe2a9143c368f6882558d797805535b25482fa5200b904be7cd34354aa478b9467ab0908499bdc747f3b99efa2fdc437eea64ef86fa8e222f21702ad2b7d0e889d5d62ee738270869f7c13fb47ec101be36e018d637a68ce4d73cb21f70f9d03e85486c4c64348d0a1295d8e54b7b4741bbab3c5e0259253a2546525553b7a1b06b8fb89d6e4e37664d88f5e6fdace40295dcba1438476a9066334396687156aa5930f312875f535422da846c29b64fd9ff66bd4955fc8f29de6e6aeca226e5a637f8f5bd10a3d71aecd423f6704f95b938fcddb4f1496d19ef2335a6f28a08550ac9ca8fe0a228c0b742247415a69f038e064a3c00ba57de653a59650ad6b7e0ca4c9d764a49da499cd81bcc47d337704bd6bfb98ef4aa2fc09ef75adab634a03ef2f9d7269f269dde183bc7f5bb67396ead28e460726837fcd2bd4503695fec2f1a53caa7a1d91c89dd86ec954fe68abf017e15f224a73c37138a2b0b0df54624b71d94e1f3a8a7733854a57ab56c9c6c713cc2e8e565939b88229dbb24f0e9f3c5850e84de0d1aa4836a2afd630425708fc3d5c5f31df667be2b0861eebb8f0f63b229bc34921e9986cc1e65d1661bd581eee7227c78ec021bbc2c44398f7dcfce37914012b09ed686f3ae6a5be51d845bead10cc12fe46c9a7754acebf1a88f53b1e07cf9538f6192dbcf90d97c04300c193a1a71445a1a35863592378b2bc5fbd9c9708e8fadbad5b3306f9be8f26462b39fb9f72b082a4d8c547fe5c1ea6f27e36a43c26cded2bc2b4f711c3798d95b40d96097995ba3175712e96ebddddb403300004f01c7087e1bb52a8c34d31e1120a4c50ae59bf985aedb30e8d3e5901c5dbb721a7b8a4d9f3870f8cd90d79ca61459baaeb6c547aa677ff93e4ee90821bad3a42707677ee93590161a48fffba7e3ddd9d67f0d3207145a6128832aece2daf83a06dd4169aad5fc89792ea234c0bb50065e502eaccec178f82b8c74a5a9ad6287b6f77240f0611b7c4b8f1c31cee2bbc1e4f9028300ac5d604731c199fc4362a1926eb79e7dd3fb4141ca97b8e92a49d2ad8dd9f77d85145a7077598b5258f74790018f641d2a71a8f79ca6891d6c41d71bb8db21130f712f57e3407b0704f479f874a11feb6885b741072316114b2e18980314ffd3ae10d9c8adf9f25887c3a8d800387bc5bbabf45663a03c756e7adedb832ff1f5a64b87c3566f38e9eb6d22e0c4e4659b60f31ffeb73822f5e68e7bbbda5191e9dc748b1c843246b46eb09ee0e55edc26e9b0c5c70bff42e568dc29e9c1a90cff3148437704dff97bcc43325c0c6febb6";
    const TEST_VERY_LARGE_INT: &str = "113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140113040274929467469880280419389222560002940607015822970101697264377219490217140";
    const TEST_BYTES: &str = "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]";

    c.bench_function("classify_and_convert_int", |b| {
        b.iter(|| {
            let encoding = vec![Encoding::from("int")];
            let _ = black_box(client.classify_and_convert(encoding, TEST_INT));
        })
    });

    c.bench_function("classify_and_convert_hex", |b| {
        b.iter(|| {
            let encoding = vec![Encoding::from("hex")];
            let _ = black_box(client.classify_and_convert(encoding, TEST_HEX));
        })
    });

    c.bench_function("classify_and_convert_base64", |b| {
        b.iter(|| {
            let encoding = vec![Encoding::from("base64")];
            let _ = black_box(client.classify_and_convert(encoding, TEST_BASE64));
        })
    });

    c.bench_function("classify_and_convert_very_large_int_to_hex", |b| {
        b.iter(|| {
            let encoding = vec![Encoding::from("hex")];
            let _ = black_box(client.classify_and_convert(encoding, TEST_VERY_LARGE_INT));
        })
    });

    c.bench_function("classify_and_convert_very_large_hex_to_int", |b| {
        b.iter(|| {
            let encoding = vec![Encoding::from("int")];
            let _ = black_box(client.classify_and_convert(encoding, TEST_VERY_LARGE_HEX));
        })
    });

    c.bench_function("classify_and_convert_bytes", |b| {
        b.iter(|| {
            let encoding = vec![Encoding::from("bytes")];
            let _ = black_box(client.classify_and_convert(encoding, TEST_BYTES));
        })
    });

    // Pure convert benchmarks (skip classification step)
    c.bench_function("convert_int_to_hex", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("int"));
            let output_enc = black_box(Encoding::from("hex"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_INT));
        })
    });

    c.bench_function("convert_hex_to_int", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("hex"));
            let output_enc = black_box(Encoding::from("int"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_HEX));
        })
    });

    c.bench_function("convert_hex_to_base64", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("hex"));
            let output_enc = black_box(Encoding::from("base64"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_HEX));
        })
    });

    c.bench_function("convert_base64_to_hex", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("base64"));
            let output_enc = black_box(Encoding::from("hex"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_BASE64));
        })
    });

    c.bench_function("convert_int_to_base58", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("int"));
            let output_enc = black_box(Encoding::from("base58"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_INT));
        })
    });

    c.bench_function("convert_very_large_int_to_hex", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("int"));
            let output_enc = black_box(Encoding::from("hex"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_VERY_LARGE_INT));
        })
    });

    c.bench_function("convert_very_large_hex_to_int", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("hex"));
            let output_enc = black_box(Encoding::from("int"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_VERY_LARGE_HEX));
        })
    });

    c.bench_function("convert_array_bytes_to_hex", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("bytes"));
            let output_enc = black_box(Encoding::from("hex"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_BYTES));
        })
    });

    // String handling benchmarks to test Cow<str> optimizations
    const TEST_MIXED_CASE: &str = "0xAbCdEf123456";
    const TEST_LOWERCASE: &str = "0xabcdef123456";

    c.bench_function("convert_mixed_case_hex", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("HEX")); // Uppercase to test Cow
            let output_enc = black_box(Encoding::from("int"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_MIXED_CASE));
        })
    });

    c.bench_function("convert_lowercase_hex", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("hex")); // Lowercase to test Cow
            let output_enc = black_box(Encoding::from("int"));
            let _ = black_box(client.convert(&input_enc, output_enc, TEST_LOWERCASE));
        })
    });

    // Padding operation benchmarks to test our optimizations
    const LARGE_BYTE_ARRAY: &str = "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20]";

    c.bench_function("convert_large_byte_array", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("bytes"));
            let output_enc = black_box(Encoding::from("base64"));
            let _ = black_box(client.convert(&input_enc, output_enc, LARGE_BYTE_ARRAY));
        })
    });

    // Nested array benchmark to test flatten_values optimization
    const NESTED_ARRAY: &str = "[[0x01, 0x02], [0x03, 0x04], [0x05, 0x06]]";

    c.bench_function("convert_nested_array", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("bytes"));
            let output_enc = black_box(Encoding::from("hex"));
            let _ = black_box(client.convert(&input_enc, output_enc, NESTED_ARRAY));
        })
    });

    // Benchmark small conversions to test overhead
    const SMALL_HEX: &str = "0xff";
    const SMALL_INT: &str = "255";

    c.bench_function("convert_small_hex_to_int", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("hex"));
            let output_enc = black_box(Encoding::from("int"));
            let _ = black_box(client.convert(&input_enc, output_enc, SMALL_HEX));
        })
    });

    c.bench_function("convert_small_int_to_base64", |b| {
        b.iter(|| {
            let input_enc = black_box(Encoding::from("int"));
            let output_enc = black_box(Encoding::from("base64"));
            let _ = black_box(client.convert(&input_enc, output_enc, SMALL_INT));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
