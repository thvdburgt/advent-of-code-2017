use std::str::FromStr;

pub fn solve_puzzle_part_1(input: &str) -> String {
    // parse the input into a list of firewall layers
    let layers: Vec<FirewallLayer> = input
        .lines()
        .map(|line| line.parse().expect("could not parse layer"))
        .collect();

    // for each layer calculate where in the range the scanner will be at the
    // picosecond we enter the layer
    let severity: u32 = layers
        .iter()
        .map(|layer| {
            if layer.scanner_pos(layer.depth) == 0 {
                layer.depth * layer.range
            } else {
                0
            }
        })
        .sum();

    severity.to_string()
}

pub fn solve_puzzle_part_2(input: &str) -> String {
    // parse the input into a list of firewall layers
    let layers: Vec<FirewallLayer> = input
        .lines()
        .map(|line| line.parse().expect("could not parse layer"))
        .collect();

    // keep increasing the delay by a picosecond until we find one where none
    // of the firewall layers will catch us
    (0..)
        .filter(|delay| {
            !layers
                .iter()
                .any(|layer| layer.scanner_pos(delay + layer.depth) == 0)
        })
        .nth(0)
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone)]
struct FirewallLayer {
    depth: u32,
    range: u32,
}

impl FirewallLayer {
    fn new(depth: u32, range: u32) -> FirewallLayer {
        assert!(range > 0);
        FirewallLayer { depth, range }
    }

    fn scanner_pos(&self, time: u32) -> u32 {
        // the scanner moves down, and back up, calculate all steps a scanner
        // van make before returning to its starting position
        let steps_in_range = if self.range <= 2 {
            self.range
        } else {
            2 * self.range - 2
        };

        // not totally correct, the way back is mapped to positions greater
        // than the range, but we are only interesed if it is 0.
        let scanner_pos = time % steps_in_range;

        let last_pos = self.range - 1;
        if scanner_pos > last_pos {
            // scanner on the way back
            last_pos - (scanner_pos - last_pos)
        } else {
            scanner_pos
        }
    }
}

#[derive(Debug, Clone)]
struct ParseFirewallLayerError(());

impl FromStr for FirewallLayer {
    type Err = ParseFirewallLayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let depth = words
            .next()
            .ok_or(ParseFirewallLayerError(()))?
            .trim_right_matches(':')
            .parse()
            .map_err(|_| ParseFirewallLayerError(()))?;
        let range = words
            .next()
            .ok_or(ParseFirewallLayerError(()))?
            .parse()
            .map_err(|_| ParseFirewallLayerError(()))?;

        Ok(FirewallLayer::new(depth, range))
    }
}
