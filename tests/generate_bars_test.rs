// use waveseeker::domain::{Bar, Tick};

// #[test]
// fn test_generate_bars() {
//     let mut ticks: Vec<Tick> = vec![
//         Tick{ timestamp: "2024-12-05T20:30:00.000000Z".parse().unwrap(), symbol: "ES".parse().unwrap(), price: 100.00, volume: 1.0 },
//         Tick{ timestamp: "2024-12-05T20:31:00.000000Z".parse().unwrap(), symbol: "ES".parse().unwrap(), price: 102.00, volume: 1.0 },
//         Tick{ timestamp: "2024-12-05T20:32:00.000000Z".parse().unwrap(), symbol: "ES".parse().unwrap(), price: 99.00, volume: 2.0 },
//         Tick{ timestamp: "2024-12-05T20:33:00.000000Z".parse().unwrap(), symbol: "ES".parse().unwrap(), price: 104.00, volume: 4.0 },
//         Tick{ timestamp: "2024-12-05T20:34:00.000000Z".parse().unwrap(), symbol: "ES".parse().unwrap(), price: 103.00, volume: 3.0 },
//     ];
//     let actual: Vec<Bar> = generate_from_ticks(&mut ticks, &4usize);

//     assert_eq!(2, actual.len());
//     assert_eq!(Bar{ timestamp: "2024-12-05T20:32:00.000000Z".parse().unwrap(), open: 100.00, high: 102.00, low: 99.00, close: 99.0, volume: 4.0 }, *actual.get(0).unwrap());
//     assert_eq!(Bar{ timestamp: "2024-12-05T20:33:00.000000Z".parse().unwrap(), open: 99.00, high: 104.00, low: 99.00, close: 104.0, volume: 4.0 }, *actual.get(1).unwrap());
// }
