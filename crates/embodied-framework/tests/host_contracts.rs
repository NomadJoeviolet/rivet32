use embodied_framework::{
    core::{
        can::{CanFrame, Id},
        init::{InitError, InitRegistry, InitStage, RegisterError},
        time::{Duration, Instant, Periodic, TimeError},
    },
    devices::{Connection, Error, dji},
};

#[test]
fn init_failure_returns_no_ready_token_and_prevents_rerun() {
    let mut registry = InitRegistry::<Vec<InitStage>, &'static str, 2>::new();
    registry
        .register(InitStage::Device, "failing-device", |_| {
            Err("device unavailable")
        })
        .unwrap();
    registry
        .register(InitStage::Late, "must-not-run", |_| {
            panic!("a hook after the failed stage must not run")
        })
        .unwrap();
    let mut stages = Vec::new();
    let result = registry.run(&mut stages, |stage, stages| {
        stages.push(stage);
        Ok(())
    });
    assert!(matches!(
        result,
        Err(InitError::Failed {
            stage: InitStage::Device,
            hook: Some("failing-device"),
            error: "device unavailable",
        })
    ));
    assert_eq!(stages.as_slice(), &InitStage::ALL[..7]);
    assert!(matches!(
        registry.run(&mut stages, |_, _| panic!("a registry must run only once")),
        Err(InitError::AlreadyStarted)
    ));
    assert_eq!(
        registry.register(InitStage::Late, "late-registration", |_| Ok(())),
        Err(RegisterError::AlreadyStarted)
    );
}

#[test]
fn rejected_motor_feedback_preserves_last_good_state_and_freshness() {
    let mut motor = dji::Motor::new(dji::Model::M3508, 0x201, 19.0, 0.0).unwrap();
    let valid = CanFrame::new(Id::Standard(0x201), &[0x10, 0, 0, 60, 0, 0, 30, 0]).unwrap();
    motor.receive(&valid, 1_000).unwrap();
    let prior = *motor.feedback().unwrap();
    let malformed = CanFrame::new(Id::Standard(0x201), &[0x20, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert_eq!(motor.receive(&malformed, 200_000), Err(Error::Range));
    assert_eq!(motor.feedback(), Some(&prior));
    assert_eq!(motor.connection(201_001), Connection::Lost);
    assert_eq!(motor.receive(&valid, 999), Err(Error::Timestamp));
    assert_eq!(motor.feedback(), Some(&prior));
}

#[test]
fn periodic_overflow_leaves_the_schedule_unchanged() {
    let mut schedule =
        Periodic::new(Instant::from_micros(u64::MAX - 3), Duration::from_micros(2)).unwrap();
    let before = schedule;
    assert_eq!(
        schedule.tick(Instant::from_micros(u64::MAX - 1)),
        Err(TimeError::Overflow)
    );
    assert_eq!(schedule, before);
}
