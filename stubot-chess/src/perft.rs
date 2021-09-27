use super::*;

use alloc::string::*;
use alloc::vec::Vec;

#[cfg(test)]
use std::println;

use derive_more::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq, Default, Add, AddAssign)]
pub struct Perft {
    pub nodes: u128,
    pub caps: u128,
    pub enps: u128,
    pub castles: u128,
    pub promotions: u128,
}

impl State {
    pub fn perft(&mut self, depth: u32) -> Perft {
        let mut res: Perft = Default::default();
        if depth == 0 {
            res.nodes = 1;
            return res;
        }

        for mv in self.gen_sudo_moves() {
            #[cfg(test)]
            let mut cpy = self.clone();
            self.make_move(mv);
            if self.is_legal() {
                if depth == 1 {
                    res.nodes += 1;
                    match mv.extra {
                        Some(MvExtra::Castle(_)) => res.castles += 1,
                        Some(MvExtra::EnPassant) => res.enps += 1,
                        Some(MvExtra::Promote(_)) => res.promotions += 1,
                        None => (),
                    }
                    if let Some(_) = mv.capture {
                        res.caps += 1;
                    }
                } else {
                    res += self.perft(depth - 1);
                }
            }
            self.unmake_move();

            // kinda expensive unmake comparison test
            #[cfg(test)]
            if *self != cpy {
                println!("orig:");
                println!("{}", cpy.board_string());
                println!("then move {}:", mv);
                println!("{:?}", mv);
                cpy.make_move(mv);
                println!("{}", cpy.board_string());
                println!("unmade into:");
                println!("{}", self.board_string());
                assert!(false);
            }
        }
        res
    }

    pub fn perftree(&mut self, depth: u32) -> String {
        let mut sum: u128 = 0;
        let mut moves: Vec<_> = self
            .gen_moves()
            .iter()
            .map(|&mv| {
                self.make_move(mv);
                let nodes = self.perft(depth - 1).nodes;
                self.unmake_move();

                sum += nodes;
                (mv.to_string(), nodes)
            })
            .collect();

        moves.sort_by_key(|tup| tup.0.to_string());

        format!(
            "{}\n\n{}",
            moves
                .iter()
                .map(|(mv, n)| format!("{} {}", mv, n))
                .collect::<Vec<_>>()
                .join("\n"),
            sum
        )
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    pub const KIWIPETE: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    pub const POS_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    pub const POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    pub const POS_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    pub const POS_6: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
    pub const DUB_M8: &str = "r3kb2/pp2qp2/2n2B2/8/2B1P3/2N2r2/PPPQ3P/2KR3R b q - 0 16";

    fn test_position(state: &mut State, nodes: Vec<u128>) {
        use pretty_assertions::assert_eq;
        for (d, &n) in nodes.iter().enumerate() {
            let count = state.perft((d + 1) as u32).nodes;
            assert_eq!(n, count);
        }
    }
    fn test_precise(state: &mut State, nodes: Vec<Perft>) {
        use pretty_assertions::assert_eq;
        for (d, &n) in nodes.iter().enumerate() {
            let count = state.perft((d + 1) as u32);
            assert_eq!(n, count);
        }
    }

    #[test]
    fn test_initial() {
        let mut state: State = Default::default();
        test_position(&mut state, vec![20, 400, 8902]);
    }

    #[test]
    fn test_kiwipete() {
        let mut state: State = str::parse(KIWIPETE).unwrap();
        test_precise(
            &mut state,
            vec![
                Perft {
                    nodes: 48,
                    caps: 8,
                    enps: 0,
                    castles: 2,
                    promotions: 0,
                },
                Perft {
                    nodes: 2039,
                    caps: 351,
                    enps: 1,
                    castles: 91,
                    promotions: 0,
                },
                Perft {
                    nodes: 97862,
                    caps: 17102,
                    enps: 45,
                    castles: 3162,
                    promotions: 0,
                },
            ],
        );
    }
    #[test]
    fn test_pos_3() {
        let mut state: State = str::parse(POS_3).unwrap();
        test_position(&mut state, vec![14, 191, 2812, 43238]);
    }
    #[test]
    fn test_pos_4() {
        let mut state: State = str::parse(POS_4).unwrap();
        test_position(&mut state, vec![6, 264, 9467]);
    }
    #[test]
    fn test_pos_5() {
        let mut state: State = str::parse(POS_5).unwrap();
        test_position(&mut state, vec![44, 1486, 62379]);
    }
    #[test]
    fn test_pos_6() {
        let mut state: State = str::parse(POS_6).unwrap();
        test_position(&mut state, vec![46, 2079, 89890]);
    }
    // none of the 6 test positions have imminent mate threats in the first 3
    // nodes. add a game that has mate threats so it can be counted
    #[test]
    fn test_dub_m8() {
        let mut state: State = str::parse(DUB_M8).unwrap();
        test_position(&mut state, vec![36, 1715, 62457]);
    }
}
