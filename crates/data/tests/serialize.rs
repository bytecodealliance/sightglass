use sightglass_data::{EffectSize, Engine, Phase};

#[test]
fn effect_size_serialized_to_csv() {
    let mut writer = csv::WriterBuilder::default()
        .has_headers(true)
        .from_writer(vec![]);
    writer
        .serialize(EffectSize {
            arch: "x86_64".into(),
            wasm: "benchmark.wasm".into(),
            phase: Phase::Execution,
            event: "cycles".into(),
            a_engine: "control.so".into(),
            a_mean: 100.0,
            b_engine: Engine {
                name: "feature.so".into(),
                flags: Some("-Wfoo=bar".into()),
            },
            b_mean: 110.0,
            significance_level: 0.05,
            half_width_confidence_interval: 1.3,
        })
        .unwrap();
    let csv = writer.into_inner().unwrap();
    let csv = String::from_utf8(csv).unwrap();
    assert_eq!(
        csv,
        "arch,wasm,phase,event,a_engine,a_engine_flags,a_mean,b_engine,b_engine_flags,b_mean,significance_level,half_width_confidence_interval\n\
         x86_64,benchmark.wasm,Execution,cycles,control.so,,100.0,feature.so,-Wfoo=bar,110.0,0.05,1.3\n"
    );
}
