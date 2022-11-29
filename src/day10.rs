/*
--- Day 10: Adapter Array ---

Patched into the aircraft's data port, you discover weather forecasts of a massive tropical storm. Before you can figure out whether it will impact your vacation plans,
however, your device suddenly turns off!

Its battery is dead.

You'll need to plug it in. There's only one problem: the charging outlet near your seat produces the wrong number of jolts. Always prepared, you make a list of all of the
joltage adapters in your bag.

Each of your joltage adapters is rated for a specific output joltage (your puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still
produce its rated output joltage.

In addition, your device has a built-in joltage adapter rated for 3 jolts higher than the highest-rated adapter in your bag. (If your adapter list were 3, 9, and 6, your
    device's built-in adapter would be rated for 12 jolts.)

Treat the charging outlet near your seat as having an effective joltage rating of 0.

Since you have some time to kill, you might as well test all of your adapters. Wouldn't want to get to your resort and realize you can't even charge your device!

If you use every adapter in your bag at once, what is the distribution of joltage differences between the charging outlet, the adapters, and your device?

For example, suppose that in your bag, you have adapters with the following joltage ratings:

16
10
15
5
1
11
7
19
6
12
4
With these adapters, your device's built-in joltage adapter would be rated for 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.

Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use every adapter, you'd need to choose them like this:

The charging outlet has an effective rating of 0 jolts, so the only adapters that could connect to it directly would need to have a joltage rating of 1, 2, or 3 jolts.
Of these, only one you have is an adapter rated 1 jolt (difference of 1).
From your 1-jolt rated adapter, the only choice is your 4-jolt rated adapter (difference of 3).
From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid choices. However, in order to not skip any adapters, you have to pick the adapter rated 5 jolts
(difference of 1).
Similarly, the next choices would need to be the adapter rated 6 and then the adapter rated 7 (with difference of 1 and 1).
The only adapter that works with the 7-jolt rated adapter is the one rated 10 jolts (difference of 3).
From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12 (difference of 1).
After 12, only valid adapter has a rating of 15 (difference of 3), then 16 (difference of 1), then 19 (difference of 3).
Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
In this example, when using every adapter, there are 7 differences of 1 jolt and 5 differences of 3 jolts.

Here is a larger example:

28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
In this larger example, in a chain that uses all of the adapters, there are 22 differences of 1 jolt and 10 differences of 3 jolts.

Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter and count the joltage differences between the charging outlet,
the adapters, and your device. What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?

2080

--- Part Two ---

To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the
charging outlet to your device. The previous rules about when adapters can successfully connect still apply.

The first example above (the one that starts with 16, 10, 15) supports the following arrangements:
(0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
(0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
(0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
(0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
(The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect
the charging outlet to your device is 8.

The second example above (the one that starts with 28, 33, 18) has many arrangements. Here are a few:

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)

(0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
46, 48, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
46, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
47, 48, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
47, 49, (52)

(0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
48, 49, (52)
In total, this set of adapters can connect the charging outlet to your device in 19208 distinct arrangements.

You glance back down at your bag and try to remember why you brought so many adapters; there must be more than a trillion valid ways to arrange them! Surely, there must be
an efficient way to count the arrangements.

What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?

6908379398144
*/

#[derive(Debug, PartialEq)]
struct ResultingVolts {
    jolt_1: usize,
    jolt_2: usize,
    jolt_3: usize,
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|e| {
            e.parse()
                .unwrap_or_else(|_| panic!("could not parse {}", e))
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[usize]) -> usize {
    let jolts = find_jolts(input);
    jolts.jolt_1 * jolts.jolt_3
}

fn find_jolts(input: &[usize]) -> ResultingVolts {
    let mut sorted_adapters = input.to_vec();
    sorted_adapters.sort_unstable();

    let mut last_voltage: usize = 0;
    let mut resulting_voltages = ResultingVolts {
        jolt_1: 0,
        jolt_2: 0,
        jolt_3: 1, //the device's adapter is always 3 higher!
    };
    for adapter in sorted_adapters {
        match adapter - last_voltage {
            1 => resulting_voltages.jolt_1 += 1,
            2 => resulting_voltages.jolt_2 += 1,
            3 => resulting_voltages.jolt_3 += 1,
            _ => panic!(
                "no fitting adapter found! Last voltage {}, next adapter {}",
                last_voltage, adapter
            ),
        }
        last_voltage = adapter;
    }
    resulting_voltages
}

fn combinations_hardcoded(input: &[usize]) -> usize {
    let mut sorted_adapters = input.to_vec();
    sorted_adapters.push(0); //add the outlet
    sorted_adapters.sort_unstable();

    sorted_adapters
        .windows(2)
        .collect::<Vec<_>>()
        .split(|n| n[1] - n[0] == 3)
        .inspect(|a| println!("chunks {:?} with a length of {}", a, a.len()))
        .map(|n| match n.len() {
            6 => 24,
            5 => 13,
            4 => 7,
            3 => 4,
            2 => 2,
            1 => 1,
            0 => 1,
            _ => panic!("needs extension!"),
        })
        .inspect(|a| println!("{}", a))
        .product::<usize>()
}

fn combinations_counted(input: &[usize]) -> usize {
    let mut sorted_adapters = input.to_vec();
    sorted_adapters.push(0); //add the outlet
    sorted_adapters.sort_unstable();
    // split the sequence where distance == 3
    sorted_adapters
        .windows(2)
        .collect::<Vec<_>>()
        .split(|n| n[1] - n[0] == 3)
        .map(count_pathes)
        .product()
}

fn count_pathes(subsequence: &[&[usize]]) -> usize {
    match subsequence.len() {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => check_for_alt_path(subsequence)
            .expect("length is minimum 3, so this error should not be possible!"),
    }
}

// [1,2] 1 2 = len 1, alt_path: 0
// [1,2] [2,3] 1 2 3 = len 2, alt_path: 1

// [1,2] [2,3] [3,4] 1 2 3 4 = len 3, alt_path: 3
// [1,2] [2,3] [3,4] [4,5] 1 2 3 4 5 = len 4, alt_path: 6

// [1,2] [2,3] [3,4] [4,5] 1 2 3 4 5 = len 4, alt_path: 6
// [1] [1] [1] [1] 1 2 3 4 5 = len 4, alt_path: 6

// [1] [1] [1 - 1] 1 2 3 x 5 = len 4, alt_path: 6
// [1] [1 - 1] [1] 1 2 x 4 5 = len 4, alt_path: 6
// [1] [1 - 1 - 1] 1 2 x x 5 = len 4, alt_path: 6
// [1 - 1] [1] [1] 1 x 3 4 5 = len 4, alt_path: 6
// [1 - 1] [1 - 1] 1 x 3 x 5 = len 4, alt_path: 6
// [1 - 1 - 1] [1] 1 x x 4 5 = len 4, alt_path: 6

// [1,2] [2,3] [3,5] [5,6] 1 2 3 5 6 = len 4, alt_path: 4

// [1,2] [2,3] [3,5] [5,6] 1 2 3 x 6 = len 4, alt_path: 4
// [1,2] [2,3] [3,5] [5,6] 1 2 x 5 6 = len 4, alt_path: 4
// [1,2] [2,3] [3,5] [5,6] 1 x 3 5 6 = len 4, alt_path: 4
// [1,2] [2,3] [3,5] [5,6] 1 x 3 x 6 = len 4, alt_path: 4

// [1] [1] [2] [1] 1 2 3 5 6 = len 4, alt_path: 4

// [1] [1] [2 - 1] 1 2 3 x 6 = len 4, alt_path: 4
// [1] [1 - 2] [1] 1 2 x 5 6 = len 4, alt_path: 4
// [1 - 1] [2] [1] 1 x 3 5 6 = len 4, alt_path: 4
// [1 - 1] [2 - 1] 1 x 3 x 6 = len 4, alt_path: 4

fn check_for_alt_path(subsequence: &[&[usize]]) -> Option<usize> {
    let mut alt_path = 0;
    println!("{:?}", subsequence);
    //convert to distances
    //[[4, 5], [5, 6], [6, 7]]
    let distances = subsequence
        .iter()
        .map(|tuple| tuple[1] - tuple[0])
        .collect::<Vec<_>>();
    let one_sequences = distances
        .split(|n| *n == 2)
        .inspect(|a| println!("alt path of {:?}", a))
        .collect::<Vec<_>>();
    alt_path += one_sequences.len();

    // let slices = distances
    //     .windows(3)
    //     .collect::<Vec<_>>()
    //     .iter()
    //     .map(|subsequence| count_pathes_a(subsequence));

    println!("distances: {:?}, alt pathes {}", distances, alt_path);
    println!("ones: {:?}", one_sequences);

    let one_distanes: usize = one_sequences
        .iter()
        // .inspect(|a| println!("one sequences {:?}", a))
        // .collect::<Vec<_>>();
        .map(|a| binary_combinations(a))
        .product();

    alt_path += one_distanes;

    // 1-1-2-1-1 =>2
    // [1-1][][1-1] =>2
    //  1-x-3-1-1
    //  1-1-3-x-1
    // 1-2-2-1-1 => 1
    // [1][][][1-1] => 1
    //  1-2-3-1
    // 1-1-2-2-1 => 1
    // [1-1][][][1] => 1
    //  1-3-2-1
    // 1-2-2-2-1 => 0
    // [1][][][][1] => 0
    //  1-2-2-2-1

    // for (i, tuple) in subsequence.iter().enumerate() {
    //     if tuple == subsequence.first()? {
    //         continue;
    //     }
    //     (*subsequence[i-1])[0] -
    //     (*subsequence[i][1]) > 3
    // }
    // //        count_pathes(&subsequence[..subsequence.len()]),
    Some(alt_path)
}

fn binary_combinations(one_sequence: &[usize]) -> usize {
    match one_sequence.len() {
        0 => 0,
        1 => 0,
        2 => 3,
        3 => 7,
        _ => binary_combinations(&one_sequence[1..]),
    }
}

fn is_path_ok(subsequence: &[usize]) -> Option<bool> {
    if (*subsequence.get(subsequence.len())? - *subsequence.get(subsequence.len())?) > 3 {
        return Some(false);
    }
    Some(true)
}

// 1-1-1-1-1
// 1-x-2-x-2

// 1 2 3 4
// 1 2 x 4
// 1 x 3 4
// 1 x x 4 !

// 1 3 4 5
// 1 3 x 5
// 1 x 4 5 !
// 1 3 4 5
// 1 3 4 5
// ==> abstand 1 auf beiden seiten ! kann entfernt werden
// 1 2 5 6
// 1 x 5 6 !
// 1 x 4 5 !

//1 2 3 4 5
//1 x 3 4 5 ok
//1 2 x 4 5 ok
//1 2 3 x 5 ok
//1 x x 4 5 ! ok
//1 x 3 x 5 ok

// [1,2] 1 2 = len 1, alt_path: 0
// [1,2] [2,3] 1 2 3 = len 2, alt_path: 1

// [1,2] [2,3] [3,4] 1 2 3 4 = len 3, alt_path: 3
// [1,2] [2,3] [3,4] [4,5] 1 2 3 4 5 = len 4, alt_path: 6

//1 2 3 4 5 ok

//1 2 3 x 5 ok
//1 2 x x 5 ok
//1 x 3 x 5 ok

//1 2 x 4 5 ok
//1 x x 4 5 ok

//1 x 3 4 5 ok

//1 2 3 4 5 ok

// 0    => 1
// 0 1  => 1
// 0 1 2 => 1
// 0 x 2 => 1+1 = 2
// 0 1 2 3 => 1
//     2 3 => 2+1
// 0 1 x 3 => 2+1
// 0 1 x 3 => 2+1

fn combinations_recursive_o_n(input: &[usize]) -> usize {
    let mut sorted_adapters = input.to_vec();

    match sorted_adapters.len() {
        0 | 1 => 1,
        2 => {
            // only if the higher number can be reached from the start (0) there is an alternative path
            if sorted_adapters[0].abs_diff(sorted_adapters[1]) <= 3 {
                2
            } else {
                1
            }
        }
        _ => {
            // sorted_adapters.push(0); //add the outlet
            sorted_adapters.sort_unstable();

            println!("adapter sequence: 0-{:?}", sorted_adapters);

            // set up first three nodes
            let mut n_1 = sorted_adapters[1];
            let mut n_2 = sorted_adapters[0];
            let mut n_3 = 0_usize;
            let mut pathes_to_n_1 = 2_usize;
            let mut pathes_to_n_2 = 1_usize;
            let mut pathes_to_n_3 = 1_usize;

            // find ways to reach the last asapter, going through each adapter
            let mut pathes_to_adapter = 0_usize;
            sorted_adapters[2..].iter().for_each(|adapter| {
                pathes_to_adapter = pathes_to_n_1;
                if adapter - n_2 <= 3 {
                    pathes_to_adapter += pathes_to_n_2;
                }
                if adapter - n_3 <= 3 {
                    pathes_to_adapter += pathes_to_n_3;
                }
                //update pathes
                n_3 = n_2;
                n_2 = n_1;
                n_1 = *adapter;
                pathes_to_n_3 = pathes_to_n_2;
                pathes_to_n_2 = pathes_to_n_1;
                pathes_to_n_1 = pathes_to_adapter;
            });
            pathes_to_adapter
        }
    }
}

fn pathes(adapters: Vec<usize>) -> usize {
    // calculate for each adapter the pathes to reach it.
    // for each new node, add the pathes of the nodes which can reach that node to it.

    // 0 -> 1
    // 0 1 => 1
    // 0 1 2 =>

    // match adapters.len() {
    //     0 => 1,
    //     1 => 1,
    //     2 =>

    // }

    //     for (index, adapter) in sorted_adapters.iter().enumerate() {
    //         println!(
    //             "\nlooking at adapter number {} with value {}",
    //             index, adapter
    //         );

    //     	if (index + 2 < sorted_adapters.len() && sorted_adapters[i+2] - sorted_adapters[i] <= 3){
    //             //			adapters[i+2].paths += adapters[i].paths;
    //             pathes +=
    //         }
    //         if (index + 3 < sorted_adapters.len() && sorted_adapters[i+3] - sorted_adapters[i] <= 3){
    // //			adapters[i+3].paths += adapters[i].paths;
    //         }
    //             //check the next three

    //             println!(
    //                 "can I skipp to the adapter {}, which has a distance of {}?",
    //                 skipp_to_adapter,
    //                 skipp_to_adapter - adapter
    //             );

    //             if skipp_to_adapter - adapter <= 3 {
    //                 pathes += match i {
    //                     2 => 1, // 1 way to skipp over one adaptor
    //                     3 => 3, // 3 ways to skipp over two adaptors
    //                     _ => panic!("can't happen"),
    //                 };

    //                 println!(
    //                     "yes, can skipp adapter {}! Number of Pathes: {}",
    //                     sorted_adapters[index + 1],
    //                     pathes
    //                 );
    //             }
    //         }
    //     }
    // pathes
    0
}

fn combinations_walked(input: &[usize]) -> usize {
    let mut pathes = 1; // one path is skipping no adapter
                        //                         //     let mut sorted_adapters = input.to_vec();
                        //                         //     sorted_adapters.push(0); //add the outlet
                        //                         //     sorted_adapters.sort_unstable();

    //     //     println!("adapter sequence: {:?}", sorted_adapters);

    //     //     for (index, adapter) in sorted_adapters.iter().enumerate() {
    //     //         println!(
    //     //             "\nlooking at adapter number {} with value {}",
    //     //             index, adapter
    //     //         );

    //     //     	if (index + 2 < sorted_adapters.len() && sorted_adapters[i+2] - sorted_adapters[i] <= 3){
    //     //             //			adapters[i+2].paths += adapters[i].paths;
    //     //             pathes +=
    //     //         }
    //     //         if (index + 3 < sorted_adapters.len() && sorted_adapters[i+3] - sorted_adapters[i] <= 3){
    //     // //			adapters[i+3].paths += adapters[i].paths;
    //     //         }
    //     //             //check the next three

    //     //             println!(
    //     //                 "can I skipp to the adapter {}, which has a distance of {}?",
    //     //                 skipp_to_adapter,
    //     //                 skipp_to_adapter - adapter
    //     //             );

    //     //             if skipp_to_adapter - adapter <= 3 {
    //     //                 pathes += match i {
    //     //                     2 => 1, // 1 way to skipp over one adaptor
    //     //                     3 => 3, // 3 ways to skipp over two adaptors
    //     //                     _ => panic!("can't happen"),
    //     //                 };

    //     //                 println!(
    //     //                     "yes, can skipp adapter {}! Number of Pathes: {}",
    //     //                     sorted_adapters[index + 1],
    //     //                     pathes
    //     //                 );
    //     //             }
    //     //         }
    //     //     }
    pathes
}

// fn combinations_walked(input: &[usize]) -> usize {
//     let mut pathes = 1; // one path is skipping no adapter
//     let mut sorted_adapters = input.to_vec();
//     sorted_adapters.push(0); //add the outlet
//     sorted_adapters.sort_unstable();

//     println!("adapter sequence: {:?}", sorted_adapters);

//     for (index, adapter) in sorted_adapters.iter().enumerate() {
//         println!(
//             "\nlooking at adapter number {} with value {}",
//             index, adapter
//         );

//         //check the next three
//         for i in [3, 2] {
//             let skipp_to_adapter = match sorted_adapters.get(index + i) {
//                 Some(adapter) => adapter,
//                 None => continue, //abort, if end of vector is reached
//             };

//             println!(
//                 "can I skipp to the adapter {}, which has a distance of {}?",
//                 skipp_to_adapter,
//                 skipp_to_adapter - adapter
//             );

//             if skipp_to_adapter - adapter <= 3 {
//                 pathes += match i {
//                     2 => 1, // 1 way to skipp over one adaptor
//                     3 => 3, // 3 ways to skipp over two adaptors
//                     _ => panic!("can't happen"),
//                 };

//                 println!(
//                     "yes, can skipp adapter {}! Number of Pathes: {}",
//                     sorted_adapters[index + 1],
//                     pathes
//                 );
//             }
//         }
//     }
//     pathes
// }

// [0, 1, 2, 3, 4, 5, 6]
// [0, x, 2, 3, 4, 5, 6]
// [0, 1, x, 3, 4, 5, 6]
// [0, x, x, 3, 4, 5, 6]
// [0, 1, 2, x, 4, 5, 6]
// [0, 1, x, x, 4, 5, 6]
// [0, 1, 2, 3, 4, 5, 6]

// [0, 1, 2, 4, 5, 6]
// [0, x, 2, 4, 5, 6]
// [0, 1, x, 4, 5, 6]
// [0, x, x, 4, 5, 6]
// [0, 1, 2, 4, x, 6]

// [0, x, 2, 4, x, 6]
// [0, 1, 2, 4, 5, 6]
// [0, 1, 2, 4, 5, 6]
// [0, 1, 2, 4, 5, 6]
// [0, 1, 2, 4, 5, 6]

// [0, 1, 2, 4, 5, 6]
// [0, 1, 2, 4, 5, 6]
// [0, 1, 2, 4, 5, 6]

#[aoc(day10, part2, hardcoded)]
fn part2(input: &[usize]) -> usize {
    combinations_hardcoded(input)
}
// #[aoc(day10, part2, counted)]
// fn part2_calced(input: &[usize]) -> usize {
//     combinations_counted(input)
// }
// #[aoc(day10, part2, walked)]
// fn part2_walked(input: &[usize]) -> usize {
//     combinations_walked(input)
// }
#[aoc(day10, part2, combinations_recursive_o_n)]
fn part2_combinations_recursive_o_n(input: &[usize]) -> usize {
    combinations_recursive_o_n(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = "16
10
15
5
1
11
7
19
6
12
4
";
    const SAMPLE_INPUT_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn test_part1_samples() {
        assert_eq!(
            ResultingVolts {
                jolt_1: 7,
                jolt_2: 0,
                jolt_3: 5
            },
            find_jolts(&parse_input(SAMPLE_INPUT_1))
        );
        assert_eq!(
            ResultingVolts {
                jolt_1: 22,
                jolt_2: 0,
                jolt_3: 10
            },
            find_jolts(&parse_input(SAMPLE_INPUT_2))
        );
    }

    /*

    */

    // 000
    // 001 000
    // 001 001
    // 001 010
    // 001 011
    // 001 110 !
    // 010
    // 011
    // 100
    // 101
    // 110
    // 111 !

    // 0000
    // 0001
    // 0010
    // 0011

    // 0100
    // 0101
    // 0110
    // 0111 !
    // => 7
    // 1000
    // 1001
    // 1010
    // 1011

    // 1100
    // 1101
    // 1110 !
    // 1111 !
    // ==> 7+6 = 13
    // 1 0000
    // 1 0001
    // 1 0010
    // 1 0011

    // 1 0100
    // 1 0101
    // 1 0110
    // 1 0111 !

    // 1 1000
    // 1 1001
    // 1 1010
    // 1 1011

    // 1 1100 !
    // 1 1101 !
    // 1 1110 !
    // 1 1111 !
    // 13 + 11 = 24
    // 10 0000
    // 10 0001
    // 10 0010
    // 10 0011

    // 10 0100
    // 10 0101
    // 10 0110
    // 10 0111 !

    // 10 1000
    // 10 1001
    // 10 1010
    // 10 1011

    // 10 1100
    // 10 1101
    // 10 1110 !
    // 10 1111 !
    // ==> 24 + 11 = 35
    // 11 0000
    // 11 0001
    // 11 0010
    // 11 0011

    // 11 0100
    // 11 0101
    // 11 0110
    // 11 0111 !

    // 11 1000 !
    // 11 1001 !
    // 11 1010 !
    // 11 1011 !

    // 11 1100 !
    // 11 1101 !
    // 11 1110 !
    // 11 1111 !
    // ==> 35 + 7 = 42

    #[test]
    fn test_part2_sample_1() {
        assert_eq!(8, combinations_hardcoded(&parse_input(SAMPLE_INPUT_1)));
    }

    #[test]
    fn test_part2_sample_2() {
        assert_eq!(19208, combinations_hardcoded(&parse_input(SAMPLE_INPUT_2)));
    }

    const SAMPLE_INPUT_3: &str = "3
4
5
6
7
9
10
13
16
";

    #[test]
    fn test_part2_sample_3() {
        assert_eq!(24, combinations_hardcoded(&parse_input(SAMPLE_INPUT_3)));
    }

    const SAMPLE_INPUT_4: &str = "3
4
6
7
10
11
13
16
19
";

    #[test]
    fn test_part2_sample_4() {
        assert_eq!(8, combinations_hardcoded(&parse_input(SAMPLE_INPUT_4)));
    }

    const SAMPLE_INPUT_5: &str = "4
5
6
7
10
13
16
19
22
24
25
26
29
33
34
35";

    #[test]
    fn test_part2_sample_5() {
        assert_eq!(112, combinations_hardcoded(&parse_input(SAMPLE_INPUT_5)));
    }
    const SAMPLE_INPUT_1_2_1: &str = "1
2
4
5
6
";
    #[test]
    fn test_part1_counted_sample_input_1_2_1() {
        assert_eq!(13, combinations_hardcoded(&parse_input(SAMPLE_INPUT_1_2_1)));
    }

    const SAMPLE_INPUT_1_2_2: &str = "1
2
3
4
5
";
    #[test]
    fn test_part1_counted_sample_input_1_2_2() {
        assert_eq!(13, combinations_hardcoded(&parse_input(SAMPLE_INPUT_1_2_2)));
    }

    #[test]
    fn test_part2_walked_sample_1() {
        assert_eq!(8, combinations_walked(&parse_input(SAMPLE_INPUT_1)));
    }

    #[test]
    fn test_part2_walked_sample_2() {
        assert_eq!(19208, combinations_walked(&parse_input(SAMPLE_INPUT_2)));
    }

    #[test]
    fn test_part2_walked_sample_3() {
        assert_eq!(24, combinations_walked(&parse_input(SAMPLE_INPUT_3)));
    }

    #[test]
    fn test_part2_walked_sample_4() {
        assert_eq!(8, combinations_walked(&parse_input(SAMPLE_INPUT_4)));
    }

    #[test]
    fn test_part2_walked_sample_5() {
        assert_eq!(112, combinations_walked(&parse_input(SAMPLE_INPUT_5)));
    }

    #[test]
    fn test_part2_walked_sample_input_1_2_1() {
        assert_eq!(13, combinations_walked(&parse_input(SAMPLE_INPUT_1_2_1)));
    }

    #[test]
    fn test_part2_walked_sample_input_1_2_2() {
        assert_eq!(13, combinations_walked(&parse_input(SAMPLE_INPUT_1_2_2)));
    }

    #[test]
    fn test_part2_counted_sample_1() {
        assert_eq!(8, combinations_counted(&parse_input(SAMPLE_INPUT_1)));
    }

    #[test]
    fn test_part2_counted_sample_2() {
        assert_eq!(19208, combinations_counted(&parse_input(SAMPLE_INPUT_2)));
    }

    #[test]
    fn test_part2_counted_sample_3() {
        assert_eq!(24, combinations_counted(&parse_input(SAMPLE_INPUT_3)));
    }

    #[test]
    fn test_part2_counted_sample_4() {
        assert_eq!(8, combinations_counted(&parse_input(SAMPLE_INPUT_4)));
    }

    #[test]
    fn test_part2_counted_sample_5() {
        assert_eq!(112, combinations_counted(&parse_input(SAMPLE_INPUT_5)));
    }

    #[test]
    fn test_part2_counted_sample_input_1_2_1() {
        assert_eq!(13, combinations_counted(&parse_input(SAMPLE_INPUT_1_2_1)));
    }

    #[test]
    fn test_part2_counted_sample_input_1_2_2() {
        assert_eq!(13, combinations_counted(&parse_input(SAMPLE_INPUT_1_2_2)));
    }

    #[test]
    fn test_part2_recursive_o_n_sample_1() {
        assert_eq!(8, combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_1)));
    }

    #[test]
    fn test_part2_recursive_o_n_sample_2() {
        assert_eq!(
            19208,
            combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_2))
        );
    }

    #[test]
    fn test_part2_recursive_o_n_sample_3() {
        assert_eq!(24, combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_3)));
    }

    #[test]
    fn test_part2_recursive_o_n_sample_4() {
        assert_eq!(8, combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_4)));
    }

    #[test]
    fn test_part2_recursive_o_n_sample_5() {
        assert_eq!(
            112,
            combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_5))
        );
    }

    #[test]
    fn test_part2_recursive_o_n_sample_input_1_2_1() {
        assert_eq!(
            13,
            combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_1_2_1))
        );
    }

    #[test]
    fn test_part2_recursive_o_n_sample_input_1_2_2() {
        assert_eq!(
            13,
            combinations_recursive_o_n(&parse_input(SAMPLE_INPUT_1_2_2))
        );
    }
}
