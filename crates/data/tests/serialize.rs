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
            a_median: 99.0,
            a_min: 95,
            a_max: 105,
            a_p25: 97.0,
            a_p75: 101.0,
            a_samples: 10,
            b_engine: Engine {
                name: "feature.so".into(),
                flags: Some("-Wfoo=bar".into()),
            },
            b_mean: 110.0,
            b_median: 109.0,
            b_min: 105,
            b_max: 115,
            b_p25: 107.0,
            b_p75: 111.0,
            b_samples: 10,
            significance_level: 0.05,
            ratio: 1.1,
            confidence_interval: Some((1.05, 1.15)),
            p_value: 0.001,
        })
        .unwrap();
    let csv = writer.into_inner().unwrap();
    let csv = String::from_utf8(csv).unwrap();
    assert_eq!(
        csv,
        "arch,wasm,phase,event,a_engine,a_engine_flags,a_mean,a_median,a_min,a_max,a_p25,a_p75,a_samples,b_engine,b_engine_flags,b_mean,b_median,b_min,b_max,b_p25,b_p75,b_samples,significance_level,ratio,confidence_interval_low,confidence_interval_high,p_value\n\
         x86_64,benchmark.wasm,Execution,cycles,control.so,,100.0,99.0,95,105,97.0,101.0,10,feature.so,-Wfoo=bar,110.0,109.0,105,115,107.0,111.0,10,0.05,1.1,1.05,1.15,0.001\n"
    );
}

/// A comparison that could not be made has no interval, which has to survive a
/// round trip rather than read as an interval of zero width.
#[test]
fn effect_size_without_a_confidence_interval_round_trips() {
    let json = serde_json::to_string(&EffectSize {
        arch: "x86_64".into(),
        wasm: "benchmark.wasm".into(),
        phase: Phase::Execution,
        event: "cycles".into(),
        a_engine: "control.so".into(),
        a_mean: 100.0,
        a_median: 100.0,
        a_min: 99,
        a_max: 101,
        a_p25: 99.5,
        a_p75: 100.5,
        a_samples: 3,
        b_engine: "feature.so".into(),
        b_mean: 110.0,
        b_median: 110.0,
        b_min: 109,
        b_max: 111,
        b_p25: 109.5,
        b_p75: 110.5,
        b_samples: 3,
        significance_level: 0.01,
        ratio: 1.1,
        confidence_interval: None,
        p_value: 0.1,
    })
    .unwrap();

    let effect_size: EffectSize<'_> = serde_json::from_str(&json).unwrap();
    assert_eq!(effect_size.confidence_interval, None);
    assert!(!effect_size.is_significant());
}
