//! Deterministic host-only control-loop smoke run; no MCU or bus driver is used.
use embodied_framework::{
    algorithms::pid::{Pid, StandardD, StandardI},
    core::{
        can::{CanFrame, Id},
        init::{InitRegistry, InitStage},
        time::{Duration, Instant, Periodic},
    },
    devices::{Connection, Error, dji},
    runtime::queue::Queue,
};

fn main() {
    let mut registry = InitRegistry::<Vec<(InitStage, bool)>, (), 2>::new();
    registry
        .register(InitStage::Device, "motor", |log| {
            log.push((InitStage::Device, true));
            Ok(())
        })
        .unwrap();
    registry
        .register(InitStage::PreCore, "startup", |log| {
            log.push((InitStage::PreCore, true));
            Ok(())
        })
        .unwrap();
    let mut log = Vec::new();
    let ready = registry
        .run(&mut log, |stage, log| {
            log.push((stage, false));
            Ok(())
        })
        .unwrap();
    let mut expected = Vec::new();
    for stage in InitStage::ALL {
        expected.push((stage, false));
        if matches!(stage, InitStage::PreCore | InitStage::Device) {
            expected.push((stage, true));
        }
    }
    assert_eq!(log, expected);

    ready.start(|| {
        let frame = CanFrame::new(Id::Standard(0x201), &[0x10, 0, 0, 60, 0, 0, 30, 0])
            .unwrap();
        let mut queue = Queue::<CanFrame, 1>::new();
        queue.try_send(frame).unwrap();
        assert_eq!(queue.try_send(frame), Err(frame));
        let received = queue.try_receive().unwrap();
        assert!(queue.try_receive().is_none());

        let mut motor = dji::Motor::new(dji::Model::M3508, 0x201, 19.0, 0.0).unwrap();
        motor.receive(&received, 1_000_000).unwrap();
        let feedback = *motor.feedback().unwrap();
        assert!((feedback.position_radians - std::f32::consts::PI).abs() < 1e-6);
        assert_eq!(feedback.speed_rpm, 60);
        assert_eq!(motor.connection(1_000_000), Connection::Connected);

        let invalid = CanFrame::new(Id::Standard(0x201), &[0x20, 0, 0, 0, 0, 0, 0, 0])
            .unwrap();
        assert_eq!(motor.receive(&invalid, 1_199_000), Err(Error::Range));
        assert_eq!(motor.feedback(), Some(&feedback));
        assert_eq!(motor.receive(&received, 999_999), Err(Error::Timestamp));
        assert_eq!(motor.connection(1_200_001), Connection::Lost);

        let mut controller = Pid::new(
            2.0,
            0.0,
            0.0,
            100.0,
            StandardI::new(10.0).unwrap(),
            StandardD::default(),
        )
        .unwrap();
        let output = controller
            .update(70.0, f32::from(feedback.speed_rpm), 0.01)
            .unwrap();
        assert_eq!(output, 20.0);
        let command = dji::group_frame(0x200, &[(motor.id(), output as i16)]).unwrap();
        assert_eq!(command.id(), Id::Standard(0x200));
        assert_eq!(command.data(), &[0, 20, 0, 0, 0, 0, 0, 0]);

        let mut schedule = Periodic::new(Instant::ZERO, Duration::from_millis(10)).unwrap();
        assert_eq!(schedule.tick(Instant::from_micros(9_999)).unwrap(), None);
        let tick = schedule.tick(Instant::from_micros(35_000)).unwrap().unwrap();
        assert_eq!(tick.scheduled, Instant::from_micros(10_000));
        assert_eq!(tick.missed, 2);
        assert_eq!(schedule.deadline(), Instant::from_micros(40_000));

        println!(
            "host PoC passed: 9 init stages, bounded CAN queue, DJI validation, PID output=20, missed ticks=2"
        );
    });
}
