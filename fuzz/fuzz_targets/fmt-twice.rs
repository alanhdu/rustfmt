#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rustfmt_nightly;

extern crate syn;

fuzz_target!(|data: &[u8]| {
    if let Ok(data)  = std::str::from_utf8(data) {
        if syn::parse_file(data).is_ok() {
            let mut config = rustfmt_nightly::Config::default();
            config.set().verbose(rustfmt_nightly::Verbosity::Quiet);
            config.set().emit_mode(rustfmt_nightly::EmitMode::Stdout);
            config.set().hide_parse_errors(true);

            let mut output1: Vec<u8> = Vec::with_capacity(data.len());
            let report1 = {
                let mut session1 = rustfmt_nightly::Session::new(config.clone(), Some(&mut output1));
                session1.format(rustfmt_nightly::Input::Text(data.to_string()))
            };

            let mut output2: Vec<u8> = Vec::with_capacity(data.len());
            let report2 = {
                let mut session2 = rustfmt_nightly::Session::new(config, Some(&mut output2));
                session2.format(rustfmt_nightly::Input::Text(data.to_string()))
            };

            assert!(output1 == output2);
            assert!(report1.is_ok() == report2.is_ok());
        }
    }
});
